import React from 'react';
import {AbsoluteFill, Audio, staticFile} from 'remotion';
import {PanoramicForest} from './scenes';

/**
 * Main Launch Composition:
 * Full 360-degree panoramic turn in a silhouette forest at dusk.
 */
export const Launch: React.FC = () => {
  return (
    <AbsoluteFill style={{backgroundColor: '#05020a'}}>
      {/* 360 Forest Camera Rotation */}
      <PanoramicForest />

      {/* Atmospheric dusk audio bed & tree whooshes */}
      <Audio src={staticFile('audio/music.wav')} volume={0.65} />
    </AbsoluteFill>
  );
};
