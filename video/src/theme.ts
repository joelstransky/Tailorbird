import {loadFont as loadDisplay} from '@remotion/google-fonts/Unbounded';
import {loadFont as loadBody} from '@remotion/google-fonts/PlusJakartaSans';

export const display = loadDisplay('normal', {weights: ['800', '900']}).fontFamily;
export const body = loadBody('normal', {weights: ['500', '700']}).fontFamily;

/**
 * Vivid Dusk Vector Illustration Palette
 * Mapped from the vector art style:
 * - Rich layered foliage (deep purple/plum, warm violet, dusky teal/indigo)
 * - Stylized trunks with bark ridges and branch cutouts (warm wood/plum-brown)
 * - Ground clearing / path / meadow (warm golden amber, peach-orange, dusky magenta-violet)
 * - Sky & clouds (dusk zenith purple to fiery sunset fuchsia/orange/yellow glow)
 */
export const Dusk = {
  // Sky
  skyZenith: '#18052e',
  skyMidUpper: '#3b0764',
  skyMid: '#581c87',
  skyHorizon: '#c026d3',
  skyGlow: '#f97316',
  sunWarmth: '#facc15',
  cloudHighlight: '#fde047',
  cloudBody: 'rgba(236, 72, 153, 0.35)',

  // Foliage - Foreground / Midground / Background vector tones
  foliageDeep: '#120524',      // Deepest forest green/plum shadow
  foliageDark: '#240a3d',      // Shadow side of canopies
  foliageMid: '#4a156e',       // Midtone purple/violet canopy
  foliageLight: '#7e22ce',     // Highlight canopy clumps
  foliageWarm: '#a21caf',      // Warm sunset rim on leaves
  
  // Conifer pine shades
  pineDeep: '#0f172a',
  pineDark: '#1e1b4b',
  pineMid: '#312e81',
  pineLight: '#4338ca',
  pineHighlight: '#6366f1',

  // Trunks & Branches (stylized vector wood with bark lines)
  trunkShadow: '#1c0826',
  trunkBase: '#2d113f',
  trunkMid: '#43185c',
  trunkHighlight: '#6b21a8',
  barkLine: '#160421',

  // Ground clearing / Path / Bushes
  groundDeep: '#18072b',
  groundMid: '#3c1158',
  groundHighlight: '#6b1778',
  pathColor: '#e11d48',       // Dusk path
  pathGlow: '#fb923c',        // Warm path center
  rockShadow: '#1e142e',
  rockBody: '#382554',
  rockHighlight: '#5b4080',

  // Typography & UI badges
  textLight: '#fff7ed',
  accentYellow: '#fde047',
  accentOrange: '#fb923c',
  accentPurple: '#e879f9',
  accentCyan: '#38bdf8',
};
