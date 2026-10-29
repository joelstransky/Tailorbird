import React from 'react';
import {Composition} from 'remotion';
import {LaunchModern} from './LaunchModern';
import {LaunchV2} from './LaunchV2';
import {Launch} from './Launch';
import {FPS, TOTAL_FRAMES} from './timing';

export const Root: React.FC = () => (
  <>
    {/* Fresh Modern Edition: Pesto theme, fluid spring motion, real screenshots, modern music */}
    <Composition
      id="LaunchModern"
      component={LaunchModern}
      durationInFrames={TOTAL_FRAMES}
      fps={FPS}
      width={1920}
      height={1080}
    />

    {/* Version 2: Panoramic Pesto Canopy */}
    <Composition
      id="LaunchV2"
      component={LaunchV2}
      durationInFrames={TOTAL_FRAMES}
      fps={FPS}
      width={1920}
      height={1080}
    />

    {/* Version 1: Original Dusk Forest */}
    <Composition
      id="Launch"
      component={Launch}
      durationInFrames={TOTAL_FRAMES}
      fps={FPS}
      width={1920}
      height={1080}
    />
  </>
);
