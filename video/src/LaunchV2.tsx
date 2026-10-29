import React from 'react';
import {AbsoluteFill, Audio, staticFile} from 'remotion';
import {PestoForest} from './PestoForest';

/**
 * LaunchV2:
 * Second Version of the Tailorbird Remotion Launch Video.
 *
 * Features:
 * - GitHub Pages Pastel Mint (Pesto) aesthetic
 * - 360-degree panoramic continuous gentle rotation
 * - Real high-resolution application screenshots in floating 2.5D perspective
 * - Live animated 1-click autofill cursor & field highlighting
 * - Integrated 3D mascot birds peeking and interacting across all scenes
 * - Regenerated bright acoustic morning soundtrack with birdsong chirps
 */
export const LaunchV2: React.FC = () => {
  return (
    <AbsoluteFill style={{backgroundColor: '#e4f7ee'}}>
      {/* 360 Pesto Mint Canopy & Showcases */}
      <PestoForest />

      {/* Regenerated Pesto Acoustic Canopy Audio Track */}
      <Audio src={staticFile('audio/music_pesto.wav')} volume={0.7} />
    </AbsoluteFill>
  );
};
