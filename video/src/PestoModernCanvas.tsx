import React from 'react';
import {AbsoluteFill, interpolate, useCurrentFrame} from 'remotion';
import {Pesto, outfitFont, monoFont} from './theme_pesto';

/**
 * Modern Pesto Canvas Background:
 * - Fluid pastel mint gradient atmosphere
 * - Subtle ambient white contour curves slowly rotating & breathing
 * - Soft luminous ambient orbs
 * - Minimal top status pill (brand name, version, offline guarantee)
 * - Zero trees, zero dark wipes, 100% clean modern product aesthetics
 */
export const PestoModernCanvas: React.FC<{children: React.ReactNode}> = ({children}) => {
  const frame = useCurrentFrame();

  // Slow ambient rotation of background contour geometry
  const rot1 = (frame * 0.12) % 360;
  const rot2 = (-frame * 0.08) % 360;
  const breath = Math.sin(frame / 35) * 20;

  // Spatial parallax drift as the viewer turns their head across screens
  const cameraYaw = interpolate(
    frame,
    [0, 220, 250, 450, 480, 680, 710, 920, 950, 1150, 1180, 1440],
    [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
    {extrapolateLeft: 'clamp', extrapolateRight: 'clamp'}
  );
  const parallaxX = -cameraYaw * 60;

  return (
    <AbsoluteFill
      style={{
        background: `linear-gradient(135deg, ${Pesto.bgZenith} 0%, ${Pesto.bgMidUpper} 28%, ${Pesto.bgMid} 65%, ${Pesto.bgHorizon} 100%)`,
        overflow: 'hidden',
      }}
    >
      {/* Background Spatial Atmosphere Layer (Parallax shift with VR Head Turn) */}
      <div
        style={{
          position: 'absolute',
          top: 0,
          left: 0,
          width: '100%',
          height: '100%',
          transform: `translateX(${parallaxX}px)`,
          pointerEvents: 'none',
        }}
      >
        {/* 1. Ambient Luminous Orbs */}
        <div
          style={{
            position: 'absolute',
            top: -100 + breath,
            left: -80,
            width: 700,
            height: 700,
            borderRadius: 9999,
            background: 'radial-gradient(circle, rgba(255, 255, 255, 0.55) 0%, rgba(228, 247, 238, 0) 70%)',
            filter: 'blur(60px)',
            pointerEvents: 'none',
          }}
        />
        <div
          style={{
            position: 'absolute',
            bottom: -150 - breath,
            right: -100,
            width: 800,
            height: 800,
            borderRadius: 9999,
            background: 'radial-gradient(circle, rgba(16, 185, 129, 0.08) 0%, rgba(180, 222, 204, 0) 70%)',
            filter: 'blur(70px)',
            pointerEvents: 'none',
          }}
        />

        {/* 2. Ambient White Rotating Contour Rings */}
        <div
          style={{
            position: 'absolute',
            top: -150,
            left: '15%',
            width: 1100,
            height: 1100,
            borderRadius: 9999,
            border: '2px solid rgba(255, 255, 255, 0.42)',
            transform: `rotate(${rot1}deg)`,
            pointerEvents: 'none',
          }}
        />
        <div
          style={{
            position: 'absolute',
            bottom: -220,
            right: '10%',
            width: 1050,
            height: 1050,
            borderRadius: 9999,
            border: '2px solid rgba(255, 255, 255, 0.35)',
            transform: `rotate(${rot2}deg)`,
            pointerEvents: 'none',
          }}
        />
      </div>

      {/* 3. Subtle Clean Status Bar at Bottom (Frameless, minimal) */}
      <div
        style={{
          position: 'absolute',
          bottom: 36,
          left: 56,
          right: 56,
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          zIndex: 50,
          pointerEvents: 'none',
        }}
      >
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 12,
          }}
        >
          <div
            style={{
              width: 10,
              height: 10,
              borderRadius: 999,
              background: Pesto.accentEmerald,
              boxShadow: '0 0 10px rgba(16, 185, 129, 0.6)',
            }}
          />
          <span
            style={{
              fontFamily: outfitFont,
              fontSize: 17,
              fontWeight: 600,
              color: Pesto.textPrimary,
              letterSpacing: -0.2,
            }}
          >
            Tailorbird
          </span>
          <span
            style={{
              fontFamily: monoFont,
              fontSize: 13,
              fontWeight: 500,
              color: Pesto.textDim,
              marginLeft: 4,
            }}
          >
            v0.2.0
          </span>
        </div>

        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 8,
            fontFamily: outfitFont,
            fontSize: 15,
            fontWeight: 600,
            color: Pesto.textSecondary,
          }}
        >
          <span style={{fontSize: 15}}>🛡️</span>
          <span>100% Offline &amp; Private</span>
        </div>
      </div>

      {/* 4. Active Scene Content */}
      <AbsoluteFill style={{zIndex: 10}}>{children}</AbsoluteFill>

      {/* 5. Minimal Corner Vignette */}
      <AbsoluteFill
        style={{
          pointerEvents: 'none',
          boxShadow: 'inset 0 0 90px rgba(28, 62, 59, 0.08)',
          zIndex: 40,
        }}
      />
    </AbsoluteFill>
  );
};
