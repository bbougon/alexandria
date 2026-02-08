use crate::collections::collections::Video;
use crate::event_bus::{Event, EventBusManager};
use crate::repositories::repositories;
use parking_lot::{Mutex, MutexGuard, RwLock};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::tokenizer::{Language, LowerCaser, Stemmer, TextAnalyzer};
use tantivy::{Index, IndexWriter, ReloadPolicy};

pub type ApplyPathRights = Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>;

pub struct SearchService<T, U>
where
    T: WriterCommit,
{
    bus_manager: RwLock<Option<EventBusManager>>,
    indexer: Box<dyn Indexer<T, U>>,
}

pub trait Indexer<T, U>: Send + Sync
where
    T: WriterCommit,
{
    fn initialize() -> Box<dyn Indexer<T, U>>
    where
        Self: Sized;
    fn index_all_videos(&self) -> Result<(), String> {
        let writer = self.retrieve_writer();
        let collections = repositories().collections().list();
        for collection in collections {
            for video in collection.videos {
                self.index_video(&video, &writer);
            }
        }
        writer.close()
    }
    fn retrieve_writer(&self) -> Writer<'_, T, U>;
    fn index_video(&self, video: &Video, writer: &Writer<'_, T, U>);
    fn search(
        &self,
        query_str: &str,
        bus_manager: &EventBusManager,
        apply_path_rights: Option<&ApplyPathRights>,
    ) -> Result<(), String>;
}

pub struct TantivyIndexer {
    index: Index,
    schema: Schema,
    writer: Mutex<IndexWriter>,
}

struct Fields<T> {
    name: T,
    artist: T,
    song: T,
    style: T,
    tags: T,
    path: T,
}

pub(crate) struct Writer<'a, T, U> {
    writer: MutexGuard<'a, T>,
    fields: Fields<U>,
}

impl<'a, T, U> Writer<'a, T, U> {
    fn close(mut self) -> Result<(), String>
    where
        T: WriterCommit,
    {
        self.writer.commit_all()
    }
}

pub(crate) trait WriterCommit {
    fn commit_all(&mut self) -> Result<(), String>;
}

impl WriterCommit for IndexWriter {
    fn commit_all(&mut self) -> Result<(), String> {
        self.commit().map(|_| ()).map_err(|e| e.to_string())
    }
}

impl Indexer<IndexWriter, Field> for TantivyIndexer {
    fn initialize() -> Box<dyn Indexer<IndexWriter, Field>> {
        let mut schema_builder = Schema::builder();
        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer("fr_stem")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();

        schema_builder.add_text_field("name", text_options.clone());
        schema_builder.add_text_field("artist", text_options.clone());
        schema_builder.add_text_field("song", text_options.clone());
        schema_builder.add_text_field("style", text_options.clone());
        schema_builder.add_text_field("tags", text_options);
        schema_builder.add_text_field("path", STORED);

        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());

        let tokenizer = TextAnalyzer::builder(tantivy::tokenizer::SimpleTokenizer::default())
            .filter(LowerCaser)
            .filter(Stemmer::new(Language::French))
            .build();
        index.tokenizers().register("fr_stem", tokenizer);

        let writer = index
            .writer(50_000_000)
            .expect("Failed to create tantivy writer");
        Box::new(Self {
            index,
            schema,
            writer: Mutex::new(writer),
        })
    }

    fn retrieve_writer(&self) -> Writer<'_, IndexWriter, Field> {
        let mut index_writer = self.writer.lock();

        let _ = index_writer
            .delete_all_documents()
            .map_err(|e| e.to_string());
        let _ = index_writer.commit().map_err(|e| e.to_string());

        Writer {
            writer: index_writer,
            fields: Fields {
                name: self.schema.get_field("name").unwrap(),
                artist: self.schema.get_field("artist").unwrap(),
                song: self.schema.get_field("song").unwrap(),
                style: self.schema.get_field("style").unwrap(),
                tags: self.schema.get_field("tags").unwrap(),
                path: self.schema.get_field("path").unwrap(),
            },
        }
    }

    fn index_video(&self, video: &Video, writer: &Writer<'_, IndexWriter, Field>) {
        let fields = &writer.fields;
        let mut doc = TantivyDocument::default();
        doc.add_text(fields.name, &video.name);
        doc.add_text(fields.artist, &video.artist);
        doc.add_text(fields.song, &video.song);

        let styles_str = video
            .style
            .iter()
            .map(|s| format!("{:?}", s))
            .collect::<Vec<_>>()
            .join(" ");
        doc.add_text(fields.style, &styles_str);

        doc.add_text(fields.tags, &video.tags.join(" "));
        doc.add_text(fields.path, &video.path.to_string_lossy());

        writer
            .writer
            .add_document(doc)
            .map_err(|e| e.to_string())
            .ok();
    }

    fn search(
        &self,
        query_str: &str,
        event_bus_manager: &EventBusManager,
        apply_path_rights: Option<&ApplyPathRights>,
    ) -> Result<(), String> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()
            .map_err(|e| e.to_string())?;

        reader.reload().map_err(|e| e.to_string())?;
        let searcher = reader.searcher();

        let name = self.schema.get_field("name").unwrap();
        let artist = self.schema.get_field("artist").unwrap();
        let song = self.schema.get_field("song").unwrap();
        let style = self.schema.get_field("style").unwrap();
        let tags = self.schema.get_field("tags").unwrap();

        let query_parser =
            QueryParser::for_index(&self.index, vec![name, artist, song, style, tags]);
        let query = query_parser
            .parse_query(query_str)
            .map_err(|e| e.to_string())?;

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(50))
            .map_err(|e| e.to_string())?;

        let collections = repositories().collections().list();

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher
                .doc::<TantivyDocument>(doc_address)
                .map_err(|e| e.to_string())?;
            let path_field = self.schema.get_field("path").unwrap();

            if let Some(path_value) = retrieved_doc.get_first(path_field).and_then(|v| v.as_str()) {
                for collection in &collections {
                    if let Some(video) = collection
                        .videos
                        .iter()
                        .find(|v| v.path.to_string_lossy() == path_value)
                    {
                        if let Some(callback) = apply_path_rights {
                            callback(video.path.to_str().unwrap())?;
                        }
                        event_bus_manager.event_bus.publish(Event {
                            event_type: "video:selected".parse().unwrap(),
                            data: serde_json::to_value(video.clone()).unwrap(),
                        });
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T, U> SearchService<T, U>
where
    T: WriterCommit,
{
    pub fn new(indexer: Box<dyn Indexer<T, U>>) -> Self {
        Self {
            bus_manager: RwLock::new(None),
            indexer,
        }
    }

    pub fn initialize(&self, bus_manager: EventBusManager) {
        *self.bus_manager.write() = Some(bus_manager);
    }

    pub fn index_all_videos(&self) -> Result<(), String> {
        self.indexer.index_all_videos()
    }

    pub fn search(
        &self,
        query_str: &str,
        allow_path: Option<&ApplyPathRights>,
    ) -> Result<(), String> {
        if let Some(bus_manager) = &*self.bus_manager.read() {
            return self.indexer.search(query_str, bus_manager, allow_path);
        }
        Ok(())
    }
}

#[cfg(test)]
mod search_service_setup {
    use crate::collections::collections::CollectionRepositoryMemory;
    use crate::infra::event_bus::memory_event_bus::MemoryEventBus;
    use crate::repositories::{with_test_repositories, Repositories, RepositoriesGuard};
    use std::sync::Arc;

    pub fn setup() -> (Arc<MemoryEventBus>, RepositoriesGuard) {
        let mem = CollectionRepositoryMemory::new();
        let _repos_guard = with_test_repositories(Repositories::new(Arc::new(mem)));
        let event_bus = Arc::new(MemoryEventBus::new());

        (event_bus, _repos_guard)
    }
}

#[cfg(test)]
mod search_service_test {
    use crate::collections::collection_builder::{a_collection, a_video};
    use crate::collections::collections::Video;
    use crate::event_bus::EventBusManager;
    use crate::repositories::repositories;
    use crate::search::search_service::search_service_setup::setup;
    use crate::search::search_service::{
        ApplyPathRights, Fields, Indexer, SearchService, WriterCommit,
    };
    use parking_lot::Mutex;
    use std::sync::Arc;

    struct DummyWriter;
    impl WriterCommit for DummyWriter {
        fn commit_all(&mut self) -> Result<(), String> {
            Ok(())
        }
    }

    struct DummyIndexer {
        pub indexed_videos: Arc<Mutex<Vec<Video>>>,
        pub writer: Mutex<DummyWriter>,
    }

    impl Indexer<DummyWriter, String> for DummyIndexer {
        fn initialize() -> Box<dyn Indexer<DummyWriter, String>>
        where
            Self: Sized,
        {
            Box::new(DummyIndexer {
                indexed_videos: Arc::new(Mutex::new(vec![])),
                writer: Mutex::new(DummyWriter),
            })
        }

        fn retrieve_writer(
            &self,
        ) -> crate::search::search_service::Writer<'_, DummyWriter, String> {
            crate::search::search_service::Writer {
                writer: self.writer.lock(),
                fields: Fields {
                    name: "name".to_string(),
                    artist: "artist".to_string(),
                    song: "song".to_string(),
                    style: "style".to_string(),
                    tags: "tags".to_string(),
                    path: "path".to_string(),
                },
            }
        }

        fn index_video(
            &self,
            _video: &Video,
            _writer: &crate::search::search_service::Writer<'_, DummyWriter, String>,
        ) {
            let mut indexed_videos = self.indexed_videos.lock();
            indexed_videos.push(_video.clone());
        }

        fn search(
            &self,
            _query_str: &str,
            _bus_manager: &EventBusManager,
            _apply_path_rights: Option<&ApplyPathRights>,
        ) -> Result<(), String> {
            todo!()
        }
    }

    #[test]
    fn index_all_videos() {
        let (_event_bus, _repositories_guard) = setup();
        let indexed_videos = Arc::new(Mutex::new(vec![]));
        let indexer = Box::new(DummyIndexer {
            indexed_videos: indexed_videos.clone(),
            writer: Mutex::new(DummyWriter),
        });
        let search_service = SearchService::new(indexer);
        repositories().collections().add(
            a_collection()
                .with_title("Collection 1")
                .add_video(&a_video().build())
                .add_video(&a_video().build())
                .build(),
        );
        repositories().collections().add(
            a_collection()
                .with_title("Collection 2")
                .add_video(&a_video().build())
                .build(),
        );

        search_service.index_all_videos().unwrap();

        assert_eq!(indexed_videos.lock().len(), 3);
    }
}
