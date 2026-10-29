import React from 'react';
import {AbsoluteFill, Audio, staticFile, useCurrentFrame} from 'remotion';
import {PestoModernCanvas} from './PestoModernCanvas';
import {
  ModernScene1Hook,
  ModernScene2DualPane,
  ModernScene3Autofill,
  ModernScene4Mcp,
  ModernScene5Pipeline,
  ModernScene6Finale,
} from './PestoModernScenes';

/**
 * LaunchModern:
 * 48-second Cinematic 3D Spatial VR Showcase for Tailorbird.
 *
 * Core Directives:
 * - Clear gap maintained between slides: exit completes before new item enters
 * - Multi-directional transitions: Center -> Left -> Up -> Left -> Right -> Down -> Depth
 * - Staggered entrance for all internal slide elements
 * - Zero rounded pills with strokes
 * - Perfectly centered slide content
 * - Brand-new custom synthesized audio track matching exact 48.0s duration
 */
export const LaunchModern: React.FC = () => {
  const frame = useCurrentFrame();

  return (
    <AbsoluteFill style={{backgroundColor: '#e4f7ee'}}>
      <PestoModernCanvas>
        {frame < 360 && <ModernScene1Hook />}
        {frame >= 370 && frame < 570 && <ModernScene2DualPane />}
        {frame >= 580 && frame < 780 && <ModernScene3Autofill />}
        {frame >= 790 && frame < 990 && <ModernScene4Mcp />}
        {frame >= 1000 && frame < 1200 && <ModernScene5Pipeline />}
        {frame >= 1210 && <ModernScene6Finale />}
      </PestoModernCanvas>

      {/* Brand-New Custom Synthesized Soundtrack */}
      <Audio src={staticFile('audio/music_modern.wav')} volume={0.8} />
    </AbsoluteFill>
  );
};
