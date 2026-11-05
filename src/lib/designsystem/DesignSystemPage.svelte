<script lang="ts">
  import { Play, Pencil, Trash2, Search } from '@lucide/svelte';
  import Button from '../components/Button.svelte';
  import IconButton from '../components/IconButton.svelte';
  import StyleTag from '../components/StyleTag.svelte';
  import Tag from '../components/Tag.svelte';
  import Input from '../components/Input.svelte';
  import EditableField from '../components/EditableField.svelte';
  import EditableChipRow from '../components/EditableChipRow.svelte';
  import Thumbnail from '../components/Thumbnail.svelte';
  import { STYLE_COLORS } from '../components/styleColor';

  // Jeux de données de démonstration (purement présentatif).
  const sampleStyles = ['Blues', 'Jazz', 'Rock', 'Folk'];
  const sampleTags = ['riff', 'solo', 'accords ouverts'];

  // Valeurs locales pour les champs éditables de la vitrine.
  let titleField = $state('Sweet Home Chicago');
  let artistField = $state('Robert Johnson');
  let songField = $state('Sweet Home Chicago');
  let inputField = $state('');

  const colorGroups: Record<string, [string, string][]> = {
    Surfaces: [
      ['surface', 'bg-surface'],
      ['surface-raised', 'bg-surface-raised'],
      ['surface-sunken', 'bg-surface-sunken'],
    ],
    Encre: [
      ['ink', 'bg-ink'],
      ['ink-muted', 'bg-ink-muted'],
      ['ink-subtle', 'bg-ink-subtle'],
    ],
    Marque: [
      ['brand', 'bg-brand'],
      ['brand-strong', 'bg-brand-strong'],
      ['brand-tint', 'bg-brand-tint'],
      ['on-brand', 'bg-on-brand'],
    ],
    Sémantique: [
      ['focus', 'bg-focus'],
      ['success', 'bg-success'],
      ['success-tint', 'bg-success-tint'],
      ['danger', 'bg-danger'],
      ['danger-tint', 'bg-danger-tint'],
    ],
    Bordures: [
      ['border', 'bg-border'],
      ['border-strong', 'bg-border-strong'],
    ],
  };

  const radii: [string, string][] = [
    ['rounded-sm', '6px'],
    ['rounded-md', '10px'],
    ['rounded-lg', '16px'],
    ['rounded-xl', '22px'],
    ['rounded-pill', '999px'],
  ];

  const shadows = ['shadow-sm', 'shadow-md', 'shadow-lg'];

  const buttonVariants = [
    'default',
    'secondary',
    'outline',
    'ghost',
    'link',
    'destructive',
  ] as const;

  const noop = () => {};
</script>

{#snippet sectionHead(title: string, desc: string)}
  <div class="mb-5">
    <h2 class="font-display text-xl font-bold tracking-[-0.01em] text-ink">
      {title}
    </h2>
    {#if desc}
      <p class="mt-1 max-w-[65ch] text-sm text-ink-muted">{desc}</p>
    {/if}
  </div>
{/snippet}

{#snippet swatch(label: string, bgClass: string)}
  <div class="flex flex-col gap-1.5">
    <div class="h-14 rounded-md border border-border {bgClass}"></div>
    <code class="font-mono text-[11px] text-ink-muted">{label}</code>
  </div>
{/snippet}

<div
  class="mx-auto flex w-full max-w-[980px] flex-col gap-12 px-[max(20px,env(safe-area-inset-left,0px))] py-8"
>
  <header>
    <p
      class="mb-1 text-[11px] font-semibold uppercase tracking-[0.1em] text-brand-strong"
    >
      Référence
    </p>
    <h1 class="font-display text-3xl font-bold tracking-[-0.02em] text-ink">
      Design system
    </h1>
    <p class="mt-2 max-w-[65ch] text-[15px] text-ink-muted">
      Tokens et composants d'Alexandria, portés en utilitaires Tailwind. Bascule le
      thème (pied de la barre latérale) pour vérifier le rendu clair / sombre.
    </p>
  </header>

  <!-- Couleurs -->
  <section>
    {@render sectionHead(
      'Couleurs',
      'Tokens de couleur exposés comme utilitaires Tailwind (réactifs au thème).'
    )}
    <div class="flex flex-col gap-6">
      {#each Object.entries(colorGroups) as [group, items]}
        <div>
          <h3 class="mb-2.5 text-[13px] font-semibold text-ink-muted">{group}</h3>
          <div class="grid grid-cols-[repeat(auto-fill,minmax(120px,1fr))] gap-3">
            {#each items as [label, bgClass]}
              {@render swatch(label, bgClass)}
            {/each}
          </div>
        </div>
      {/each}

      <div>
        <h3 class="mb-2.5 text-[13px] font-semibold text-ink-muted">
          Chips de style (6 teintes déterministes)
        </h3>
        <div class="grid grid-cols-[repeat(auto-fill,minmax(120px,1fr))] gap-3">
          {#each STYLE_COLORS as color}
            <div class="flex flex-col gap-1.5">
              <div
                class="grid h-14 place-items-center rounded-md border border-border text-[13px] font-semibold"
                style="background:var(--tag-{color}-bg);color:var(--tag-{color}-fg)"
              >
                Aa
              </div>
              <code class="font-mono text-[11px] text-ink-muted">tag-{color}</code>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </section>

  <!-- Typographie -->
  <section>
    {@render sectionHead(
      'Typographie',
      'Trois familles auto-hébergées : Bricolage Grotesque (titres), Figtree (corps) et IBM Plex Mono (données).'
    )}
    <div
      class="flex flex-col gap-5 rounded-lg border border-border bg-surface-raised p-6"
    >
      <div>
        <code class="font-mono text-[11px] text-ink-subtle"
          >font-display · Bricolage Grotesque</code
        >
        <p class="font-display text-3xl font-bold tracking-[-0.02em] text-ink">
          Retrouver un cours en un instant
        </p>
      </div>
      <div>
        <code class="font-mono text-[11px] text-ink-subtle">font-sans · Figtree</code
        >
        <p class="font-sans text-base text-ink">
          Le corps de texte par défaut, lisible et chaleureux pour les listes et les
          descriptions de vidéos.
        </p>
      </div>
      <div>
        <code class="font-mono text-[11px] text-ink-subtle"
          >font-mono · IBM Plex Mono</code
        >
        <p class="font-mono text-sm text-ink-muted">
          12:04 · 148 Mo · ~/Vidéos/guitare
        </p>
      </div>
      <div class="flex flex-col gap-1 border-t border-border pt-4">
        <h1 class="text-ink">Titre H1</h1>
        <h2 class="text-ink">Titre H2</h2>
        <h3 class="text-ink">Titre H3</h3>
        <h4 class="text-ink">Titre H4</h4>
      </div>
    </div>
  </section>

  <!-- Rayons & ombres -->
  <section>
    {@render sectionHead(
      'Rayons & ombres',
      'Échelle `--r-*` / `--sh-*` branchée sur les utilitaires `rounded-*` et `shadow-*`.'
    )}
    <div class="flex flex-col gap-6">
      <div class="grid grid-cols-[repeat(auto-fill,minmax(120px,1fr))] gap-3">
        {#each radii as [cls, px]}
          <div class="flex flex-col gap-1.5">
            <div class="h-16 border border-border-strong bg-brand-tint {cls}"></div>
            <code class="font-mono text-[11px] text-ink-muted">{cls} · {px}</code>
          </div>
        {/each}
      </div>
      <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-5">
        {#each shadows as cls}
          <div class="flex flex-col gap-2">
            <div class="h-16 rounded-lg bg-surface-raised {cls}"></div>
            <code class="font-mono text-[11px] text-ink-muted">{cls}</code>
          </div>
        {/each}
      </div>
    </div>
  </section>

  <!-- Boutons -->
  <section>
    {@render sectionHead(
      'Button',
      'Variantes et tailles (class-variance-authority). `enabled={false}` désactive.'
    )}
    <div
      class="flex flex-col gap-5 rounded-lg border border-border bg-surface-raised p-6"
    >
      <div class="flex flex-wrap items-center gap-3">
        {#each buttonVariants as variant}
          <Button {variant}>{variant}</Button>
        {/each}
      </div>
      <div class="flex flex-wrap items-center gap-3 border-t border-border pt-5">
        <Button size="sm">sm</Button>
        <Button size="default">default</Button>
        <Button size="lg">lg</Button>
        <Button enabled={false}>désactivé</Button>
      </div>
    </div>
  </section>

  <!-- Boutons à icône -->
  <section>
    {@render sectionHead(
      'IconButton',
      'Bouton à icône seule, libellé accessible obligatoire. Variantes ghost / brand / danger, tailles sm / md.'
    )}
    <div
      class="flex flex-wrap items-center gap-4 rounded-lg border border-border bg-surface-raised p-6"
    >
      <IconButton label="Lire" variant="ghost"><Play /></IconButton>
      <IconButton label="Rechercher" variant="brand"><Search /></IconButton>
      <IconButton label="Supprimer" variant="danger"><Trash2 /></IconButton>
      <IconButton label="Éditer" variant="ghost" size="sm"><Pencil /></IconButton>
    </div>
  </section>

  <!-- Chips -->
  <section>
    {@render sectionHead(
      'StyleTag & Tag',
      'Chips colorés pour les styles musicaux (couleur déterministe) et chips neutres pour les tags libres.'
    )}
    <div
      class="flex flex-col gap-5 rounded-lg border border-border bg-surface-raised p-6"
    >
      <div class="flex flex-wrap items-center gap-2">
        {#each sampleStyles as name}
          <StyleTag {name} />
        {/each}
        <StyleTag name="Blues" removable onremove={noop} />
      </div>
      <div class="flex flex-wrap items-center gap-2 border-t border-border pt-5">
        {#each sampleTags as label}
          <Tag {label} />
        {/each}
        <Tag label="à revoir" removable onremove={noop} />
      </div>
    </div>
  </section>

  <!-- Champs -->
  <section>
    {@render sectionHead(
      'Input & EditableField',
      'Champ de formulaire standard, et champ « édition au survol » (variantes titre / artiste / morceau).'
    )}
    <div
      class="flex flex-col gap-5 rounded-lg border border-border bg-surface-raised p-6"
    >
      <div class="max-w-sm">
        <Input
          placeholder="Rechercher une vidéo…"
          autofocus={false}
          bind:value={inputField}
        />
      </div>
      <div class="flex flex-col gap-1.5 border-t border-border pt-5">
        <EditableField variant="title" label="Titre" bind:value={titleField} />
        <div
          class="flex flex-wrap items-baseline gap-x-1 text-[13px] text-ink-muted"
        >
          <EditableField variant="artist" label="Artiste" bind:value={artistField} />
          <span class="text-ink-subtle">·</span>
          <EditableField variant="song" label="Morceau" bind:value={songField} />
        </div>
      </div>
    </div>
  </section>

  <!-- Lignes de chips éditables -->
  <section>
    {@render sectionHead(
      'EditableChipRow',
      'Ligne de chips en lecture seule, bascule en édition via l’icône crayon (croix de suppression + contrôle d’ajout).'
    )}
    <div
      class="flex flex-col gap-4 rounded-lg border border-border bg-surface-raised p-6"
    >
      <EditableChipRow editLabel="Modifier les styles">
        {#snippet children(editing)}
          {#each sampleStyles as name}
            <StyleTag {name} removable={editing} onremove={noop} />
          {/each}
        {/snippet}
        {#snippet add()}
          <span class="text-[12px] text-ink-subtle">+ ajouter un style</span>
        {/snippet}
      </EditableChipRow>

      <EditableChipRow editLabel="Modifier les tags">
        {#snippet children(editing)}
          {#each sampleTags as label}
            <Tag {label} removable={editing} onremove={noop} />
          {/each}
        {/snippet}
        {#snippet add()}
          <span class="text-[12px] text-ink-subtle">+ ajouter un tag</span>
        {/snippet}
      </EditableChipRow>
    </div>
  </section>

  <!-- Vignette -->
  <section>
    {@render sectionHead(
      'Thumbnail',
      'Vignette réutilisable : puits dégradé teinté par le style, bouton Lire, pastille de durée. Variantes carte et import.'
    )}
    <div
      class="flex flex-wrap items-end gap-6 rounded-lg border border-border bg-surface-raised p-6"
    >
      <div class="flex flex-col gap-2">
        <Thumbnail styleName="Blues" duration="12:04" onplay={noop} />
        <code class="font-mono text-[11px] text-ink-muted">variant=card</code>
      </div>
      <div class="flex flex-col gap-2">
        <Thumbnail variant="import" styleName="Jazz" />
        <code class="font-mono text-[11px] text-ink-muted">variant=import</code>
      </div>
    </div>
  </section>

  <!-- Composants métier -->
  <section>
    {@render sectionHead(
      'Composants métier',
      'Assemblés à partir des primitives ci-dessus, mais liés au domaine (store de collection + commandes Tauri) : documentés ici, pas rendus en isolation.'
    )}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(240px,1fr))] gap-3">
      {#each [['VideoCard', 'Carte vidéo : vignette, titre/artiste/morceau éditables, styles, tags, poids.'], ['VideoInfoPanel', 'Panneau d’infos du lecteur : titre, artiste·morceau, styles, tags, durée, taille, emplacement.'], ['VideoPlayer', 'Lecteur en modale : scène 16:9 + panneau d’infos.']] as [name, desc]}
        <div class="rounded-md border border-border bg-surface-sunken p-4">
          <code class="font-mono text-[12px] font-semibold text-ink">{name}</code>
          <p class="mt-1 text-[13px] text-ink-muted">{desc}</p>
        </div>
      {/each}
    </div>
  </section>
</div>
