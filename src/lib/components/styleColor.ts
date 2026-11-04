/**
 * Associe de façon déterministe un nom de style musical à l'une des six
 * couleurs de tag du design system. Le même nom donne toujours la même
 * couleur (contrairement à l'ancien tirage aléatoire).
 */
export const STYLE_COLORS = [
  'rose',
  'amber',
  'lime',
  'teal',
  'sky',
  'violet',
] as const;

export type StyleColor = (typeof STYLE_COLORS)[number];

export function styleColor(name: string): StyleColor {
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = (hash * 31 + name.charCodeAt(i)) >>> 0;
  }
  return STYLE_COLORS[hash % STYLE_COLORS.length];
}
