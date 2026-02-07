use crate::event_bus::{Event, EventBusManager};
use crate::repositories::repositories;
use parking_lot::{Mutex, RwLock};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::tokenizer::{Language, LowerCaser, Stemmer, TextAnalyzer};
use tantivy::{Index, IndexWriter, ReloadPolicy};
use tauri::{AppHandle, Manager, Runtime};

pub struct SearchService {
    app: RwLock<Option<AppHandle>>,
    bus_manager: RwLock<Option<EventBusManager>>,
    index: Index,
    schema: Schema,
    writer: Mutex<IndexWriter>,
}

impl SearchService {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();
        // Use 'text' for searchable fields with French indexing
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
        // Stored but not indexed field for identification
        schema_builder.add_text_field("path", STORED);

        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());

        // Setup French tokenizer
        let tokenizer = TextAnalyzer::builder(tantivy::tokenizer::SimpleTokenizer::default())
            .filter(LowerCaser)
            .filter(Stemmer::new(Language::French))
            .build();
        index.tokenizers().register("fr_stem", tokenizer);

        let writer = index
            .writer(50_000_000)
            .expect("Failed to create tantivy writer");

        Self {
            app: RwLock::new(None),
            bus_manager: RwLock::new(None),
            index,
            schema,
            writer: Mutex::new(writer),
        }
    }

    pub fn initialize(&self, bus_manager: EventBusManager, app: AppHandle) {
        *self.bus_manager.write() = Some(bus_manager);
        *self.app.write() = Some(app);
    }

    pub fn index_all_videos(&self) -> Result<(), String> {
        let mut index_writer = self.writer.lock();

        index_writer
            .delete_all_documents()
            .map_err(|e| e.to_string())?;

        // Ensure changes are visible before adding new docs
        index_writer.commit().map_err(|e| e.to_string())?;

        let collections = repositories().collections().list();

        let name = self.schema.get_field("name").unwrap();
        let artist = self.schema.get_field("artist").unwrap();
        let song = self.schema.get_field("song").unwrap();
        let style = self.schema.get_field("style").unwrap();
        let tags = self.schema.get_field("tags").unwrap();
        let path = self.schema.get_field("path").unwrap();

        for collection in collections {
            for video in collection.videos {
                let mut doc = tantivy::TantivyDocument::default();
                doc.add_text(name, &video.name);
                doc.add_text(artist, &video.artist);
                doc.add_text(song, &video.song);

                let styles_str = video
                    .style
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect::<Vec<_>>()
                    .join(" ");
                doc.add_text(style, &styles_str);

                doc.add_text(tags, &video.tags.join(" "));
                doc.add_text(path, &video.path.to_string_lossy());

                index_writer.add_document(doc).map_err(|e| e.to_string())?;
            }
        }

        index_writer.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn search(&self, query_str: &str) -> Result<(), String> {
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
                .doc::<tantivy::TantivyDocument>(doc_address)
                .map_err(|e| e.to_string())?;
            let path_field = self.schema.get_field("path").unwrap();

            if let Some(path_value) = retrieved_doc.get_first(path_field).and_then(|v| v.as_str()) {
                // Find the video in collections by path
                for collection in &collections {
                    if let Some(video) = collection
                        .videos
                        .iter()
                        .find(|v| v.path.to_string_lossy() == path_value)
                    {
                        if let Some(bus_manager) = &*self.bus_manager.read() {
                            self.allow_path(
                                &self.app.read().clone().unwrap(),
                                video.path.to_str().unwrap(),
                            )?;
                            bus_manager.event_bus.publish(Event {
                                event_type: "video:selected".parse().unwrap(),
                                data: serde_json::to_value(video.clone()).unwrap(),
                            });
                        }
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn allow_path<R: Runtime>(&self, app: &AppHandle<R>, path: &str) -> Result<(), String> {
        app.asset_protocol_scope()
            .allow_file(path)
            .map_err(|e: tauri::Error| e.to_string())
    }
}
