import {loadFont as loadDisplay} from '@remotion/google-fonts/Outfit';
import {loadFont as loadMono} from '@remotion/google-fonts/JetBrainsMono';

export const outfitFont = loadDisplay('normal', {
  weights: ['500', '600', '700'],
}).fontFamily;

export const monoFont = loadMono('normal', {
  weights: ['500', '600'],
}).fontFamily;

/**
 * Pesto Mint Theme Palette
 * Exactly matched to the Tailorbird GitHub Pages landing page.
 */
export const Pesto = {
  // Background gradient stops
  bgZenith: '#e4f7ee',
  bgMidUpper: '#d5f0e3',
  bgMid: '#c6e9d8',
  bgHorizon: '#b4decc',

  // Deep Forest Teal Typography
  textPrimary: '#1c3e3b',
  textSecondary: '#2b534f',
  textMuted: '#3d6964',
  textDim: '#57827d',

  // Borders & Outlines
  borderSubtle: 'rgba(28, 62, 59, 0.18)',
  borderMedium: 'rgba(28, 62, 59, 0.32)',
  borderFocus: '#1c3e3b',

  // Accents & Badges
  accentEmerald: '#10b981',
  accentTeal: '#0d9488',
  accentWarm: '#f97316',
  accentAmber: '#f59e0b',
  accentRose: '#f43f5e',

  // Window Dots
  dotClose: '#ef4444',
  dotMin: '#f59e0b',
  dotMax: '#10b981',

  // Card & Container Floating Shadows
  shadowLight: '0 20px 60px -15px rgba(28, 62, 59, 0.14)',
  shadowHeavy: '0 35px 90px -20px rgba(28, 62, 59, 0.22)',
};
