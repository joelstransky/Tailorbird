import React from 'react';
import {
  AbsoluteFill,
  Img,
  interpolate,
  staticFile,
  useCurrentFrame,
} from 'remotion';
import { Dusk, body } from './theme';
import { CLEARINGS, getPanAngle, TOTAL_FRAMES } from './timing';
import {
  BroadcastTitle,
  BroadcastDualPane,
  BroadcastAutofill,
  BroadcastProspects,
} from './BroadcastUI';

/**
 * 360-Degree Panoramic Forest Rotation with 3-Layer Raster Parallax
 *
 * Requirements:
 * 1. Simulating viewer turning in place (panoramic forest rotation).
 * 2. Background parallax:
 *    - Farground: 0.2x speed
 *    - Midground: 0.5x speed
 * 3. Foreground Shadow Tree:
 *    - Slower motion than the background as camera turns.
 *    - Naturally uncropped branches (organic silhouette with full branches).
 *    - Dimmed down so it sits in deep shadow, not glowing or competing with content.
 *    - Enters and exits the stage strictly in one motion during camera turns.
 *    - Does NOT linger or cover content during the clearing pause.
 * 4. Content: Clean broadcast motion graphics without generic web containers.
 * 5. Seamless 360° Loop: Frame 0 and Frame 899 match perfectly.
 */

export const PanoramicForest: React.FC = () => {
  const frame = useCurrentFrame();

  const { panAngle, currentClearingIdx, isPaused, slideProgress } = getPanAngle(frame);

  // Exact 360° circular panorama width: 12,000 px
  const PAN_WIDTH = 12000;
  const currentPanPx = (panAngle / 360) * PAN_WIDTH;

  return (
    <AbsoluteFill style={{ backgroundColor: Dusk.skyZenith, overflow: 'hidden' }}>
      {/* 1. LAYER: FARGROUND (Dusk Ridges & Sky Plate) - Parallax factor: 0.2 */}
      <FargroundParallaxLayer currentPanPx={currentPanPx * 0.2} />

      {/* 2. LAYER: MIDGROUND (Forest Pines & Meadow) - Parallax factor: 0.5 */}
      <MidgroundParallaxLayer currentPanPx={currentPanPx * 0.5} />

      {/* 3. LAYER: SLIDE CONTENTS (STATIC DURING PAUSE, FADES OUT BEFORE TURN) */}
      <SlideContentsLayer
        currentClearingIdx={currentClearingIdx}
        isPaused={isPaused}
        slideProgress={slideProgress}
      />

      {/* 4. LAYER: FOREGROUND SHADOW TREE (Sweeps completely across only during the turn) */}
      <ForegroundShadowTreeLayer frame={frame} />

      {/* 5. ATMOSPHERIC BROADCAST VIGNETTE */}
      <AbsoluteFill
        style={{
          pointerEvents: 'none',
          background: 'radial-gradient(circle at 50% 50%, transparent 55%, rgba(10, 4, 20, 0.6) 100%)',
          boxShadow: 'inset 0 0 120px rgba(10, 4, 20, 0.85)',
        }}
      />
    </AbsoluteFill>
  );
};

/* --- 1. FARGROUND PARALLAX LAYER --- */
const FargroundParallaxLayer: React.FC<{ currentPanPx: number }> = ({ currentPanPx }) => {
  const stride = 2400;
  const offsetX = -(((currentPanPx % stride) + stride) % stride);

  return (
    <div
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: stride * 3,
        height: 1080,
        transform: `translateX(${offsetX - stride}px)`,
        pointerEvents: 'none',
        display: 'flex',
      }}
    >
      {[0, 1, 2].map((g) => (
        <div
          key={g}
          style={{
            width: stride,
            height: 1080,
            position: 'relative',
            overflow: 'hidden',
          }}
        >
          <Img
            src={staticFile('art/dusk_farground.jpg')}
            style={{
              width: stride,
              height: 1080,
              objectFit: 'cover',
            }}
          />
        </div>
      ))}
    </div>
  );
};

/* --- 2. MIDGROUND PARALLAX LAYER --- */
const MidgroundParallaxLayer: React.FC<{ currentPanPx: number }> = ({ currentPanPx }) => {
  const stride = 2000;
  const offsetX = -(((currentPanPx % stride) + stride) % stride);

  return (
    <div
      style={{
        position: 'absolute',
        bottom: 0,
        left: 0,
        width: stride * 3,
        height: 1080,
        transform: `translateX(${offsetX - stride}px)`,
        pointerEvents: 'none',
        display: 'flex',
      }}
    >
      {[0, 1, 2].map((g) => (
        <div
          key={g}
          style={{
            width: stride,
            height: 1080,
            position: 'relative',
          }}
        >
          <Img
            src={staticFile('art/forest_midground.png')}
            style={{
              position: 'absolute',
              bottom: -40,
              left: 0,
              width: stride,
              height: 1120,
              objectFit: 'cover',
              filter: 'drop-shadow(0 20px 30px rgba(0,0,0,0.6))',
            }}
          />
        </div>
      ))}
    </div>
  );
};

/**
 * --- 3. FOREGROUND SHADOW TREE LAYER ---
 *
 * Behavior:
 * - Each segment is 180 frames:
 *   * Frames 0 to 110: Camera paused at clearing -> Tree is completely off-screen.
 *   * Frames 110 to 180: Camera turns to next clearing -> Tree enters from right,
 *     passes across the screen, and exits to left in ONE CONTINUOUS MOTION.
/**
 * --- 3. FOREGROUND SHADOW TREE LAYER ---
 *
 * Requirements:
 * 1. Tree sweeps across in one continuous, gentle motion (140 frames ~ 4.66s, much slower!).
 * 2. Tree enters and begins covering the slide content right as the slide content starts to fade out (frame 60).
 * 3. As the tree sweeps across and clears the screen to the left (around frame 185-195 / frames 5-15 of next segment),
 *    the new slide content transitions in right as the tree passes over it.
 * 4. This creates the exact illusion that the content was fixed in space, and the camera turned away
 *    passing behind the giant foreground tree!
 */
const ForegroundShadowTreeLayer: React.FC<{ frame: number }> = ({ frame }) => {
  const normFrame = ((frame % TOTAL_FRAMES) + TOTAL_FRAMES) % TOTAL_FRAMES;
  const segmentLength = 180;
  const frameInSegment = normFrame % segmentLength;

  // Sweep window: 140 frames (~4.66s gentle, steady motion)
  // Starts at frame 55 of the clearing, and finishes at frame 195 (frame 15 of next clearing)
  const SWEEP_START = 55;
  const SWEEP_DURATION = 140;

  let relFrame = frameInSegment - SWEEP_START;
  if (relFrame < 0) relFrame += segmentLength;

  if (relFrame >= SWEEP_DURATION) {
    // Tree is completely off-screen during the clear portion of the clearing hold
    return null;
  }

  // Progress across the sweep: 0 to 1
  const tau = relFrame / SWEEP_DURATION;
  // Smooth zero-jerk ease
  const ease = tau - Math.sin(2 * Math.PI * tau) / (2 * Math.PI);
  // Trajectory: from +2200 to -4800 (complete uncropped clearance on both sides)
  const treeX = interpolate(ease, [0, 1], [2200, -4800]);

  return (
    <AbsoluteFill style={{ pointerEvents: 'none', zIndex: 40 }}>
      <div
        style={{
          position: 'absolute',
          top: -1900, // Preserves the user's fixed scale and y-position
          left: treeX,
          width: 4410,
          height: 3780,
        }}
      >
        <Img
          src={staticFile('art/shadow_tree_full.png')}
          style={{
            width: 4410,
            height: 3780,
            objectFit: 'contain',
            filter: 'drop-shadow(-30px 15px 50px rgba(0,0,0,0.95)) brightness(0.65) contrast(1.15)',
          }}
        />
      </div>
    </AbsoluteFill>
  );
};

/* --- 4. SLIDE CONTENTS LAYER --- */
const SlideContentsLayer: React.FC<{
  currentClearingIdx: number;
  isPaused: boolean;
  slideProgress: number; // 0 to 95
}> = ({ currentClearingIdx, isPaused, slideProgress }) => {
  if (!isPaused || slideProgress < 0) return null;

  const clearing = CLEARINGS[currentClearingIdx];
  if (!clearing) return null;

  // Fade out starts at frame 68 just as the leading edge of the foreground tree covers the content,
  // completing by frame 90 as the trunk passes over.
  const fadeOutOpacity = interpolate(slideProgress, [68, 90], [1, 0], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });

  return (
    <AbsoluteFill
      style={{
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center',
        textAlign: 'center',
        pointerEvents: 'none',
        zIndex: 25,
        opacity: fadeOutOpacity,
      }}
    >
      {/* 1. Common Broadcast Title */}
      <BroadcastTitle
        title={clearing.title}
        subtitle={clearing.subtitle}
        slideProgress={slideProgress}
      />

      {/* 2. Clearing-Specific Hero Broadcast Animation */}
      {currentClearingIdx === 0 && <HeroLogoPresentation slideProgress={slideProgress} />}
      {currentClearingIdx === 1 && <TabChaosPresentation slideProgress={slideProgress} />}
      {currentClearingIdx === 2 && <BroadcastDualPane slideProgress={slideProgress} />}
      {currentClearingIdx === 3 && <BroadcastAutofill slideProgress={slideProgress} />}
      {currentClearingIdx === 4 && <BroadcastProspects slideProgress={slideProgress} />}
    </AbsoluteFill>
  );
};

/* Slide 0: Hero Logo */
const HeroLogoPresentation: React.FC<{ slideProgress: number }> = ({ slideProgress }) => {
  const scale = interpolate(slideProgress, [0, 20], [0.85, 1], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });

  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: 30,
        transform: `scale(${scale})`,
      }}
    >
      <Img
        src={staticFile('icon.png')}
        style={{
          width: 170,
          height: 170,
          borderRadius: 42,
          boxShadow: '0 25px 80px rgba(249, 115, 22, 0.6), 0 0 100px rgba(250, 204, 21, 0.4)',
          border: '3px solid rgba(255, 255, 255, 0.3)',
        }}
      />
    </div>
  );
};

/* Slide 1: Tab Chaos Visual */
const TabChaosPresentation: React.FC<{ slideProgress: number }> = ({ slideProgress }) => {
  const tabs = [
    'Job Application (Greenhouse) #12',
    'Senior Frontend Engineer - Workday',
    'LinkedIn - 1,420 Jobs Matching Search',
    'Job Tracker Spreadsheet 2026_Final_v3.xlsx',
    'Cover Letter Draft (Copy 4).docx',
  ];

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: 12,
        width: 880,
      }}
    >
      {tabs.map((tab, i) => {
        const offset = interpolate(slideProgress, [i * 3, i * 3 + 12], [40, 0], {
          extrapolateLeft: 'clamp',
          extrapolateRight: 'clamp',
        });
        const op = interpolate(slideProgress, [i * 3, i * 3 + 12], [0, 1], {
          extrapolateLeft: 'clamp',
          extrapolateRight: 'clamp',
        });

        return (
          <div
            key={tab}
            style={{
              padding: '16px 24px',
              borderRadius: 10,
              background: 'rgba(15, 6, 30, 0.75)',
              border: '1.5px solid rgba(249, 115, 22, 0.35)',
              backdropFilter: 'blur(10px)',
              display: 'flex',
              alignItems: 'center',
              gap: 16,
              transform: `translateY(${offset}px)`,
              opacity: op,
              boxShadow: '0 15px 35px rgba(0, 0, 0, 0.6)',
            }}
          >
            <div style={{ width: 10, height: 10, borderRadius: 5, background: '#ef4444' }} />
            <div
              style={{
                fontFamily: body,
                fontWeight: 700,
                fontSize: 18,
                color: '#fff',
                letterSpacing: 0.5,
              }}
            >
              {tab}
            </div>
          </div>
        );
      })}
    </div>
  );
};
