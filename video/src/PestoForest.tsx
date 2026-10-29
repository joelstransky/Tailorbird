import React from 'react';
import {
  AbsoluteFill,
  interpolate,
  staticFile,
  useCurrentFrame,
} from 'remotion';
import {Pesto, outfitFont, monoFont} from './theme_pesto';
import {CLEARINGS, getPanAngle, TOTAL_FRAMES} from './timing';
import {
  PestoTitle,
  PestoIntroShowcase,
  PestoChaosShowcase,
  PestoDualPaneShowcase,
  PestoAutofillShowcase,
  PestoPipelineShowcase,
  PestoOutroShowcase,
} from './BroadcastUI_v2';

/**
 * Pesto Mint 360-Degree Panoramic Canopy
 * Continuous gentle parallax rotation with organic pastel mint aesthetic.
 */
export const PestoForest: React.FC = () => {
  const frame = useCurrentFrame();
  const {panAngle, currentClearingIdx, isPaused, slideProgress} = getPanAngle(frame);

  const PAN_WIDTH = 12000;
  const currentPanPx = (panAngle / 360) * PAN_WIDTH;

  return (
    <AbsoluteFill
      style={{
        background: `linear-gradient(180deg, ${Pesto.bgZenith} 0%, ${Pesto.bgMidUpper} 22%, ${Pesto.bgMid} 58%, ${Pesto.bgHorizon} 100%)`,
        overflow: 'hidden',
      }}
    >
      {/* 1. LAYER: AMBIENT CONTOURS (Parallax Factor: 0.1) */}
      <PestoContoursLayer currentPanPx={currentPanPx * 0.1} />

      {/* 2. LAYER: FAR CANOPY RIDGES (Parallax Factor: 0.2) */}
      <PestoFarCanopyLayer currentPanPx={currentPanPx * 0.2} />

      {/* 3. LAYER: MIDGROUND FOREST GROVE (Parallax Factor: 0.5) */}
      <PestoMidCanopyLayer currentPanPx={currentPanPx * 0.5} />

      {/* 4. LAYER: SLIDE CONTENTS (REVEALS DURING CLEARING HOLD) */}
      <PestoSlideContentsLayer
        currentClearingIdx={currentClearingIdx}
        isPaused={isPaused}
        slideProgress={slideProgress}
        frame={frame}
      />

      {/* 5. LAYER: FOREGROUND SILHOUETTE TREE (SWEEPS ACROSS ONLY DURING SCENE TRANSITIONS) */}
      <PestoForegroundTreeLayer frame={frame} />

      {/* 6. SUBTLE SOFT BROADCAST VIGNETTE */}
      <AbsoluteFill
        style={{
          pointerEvents: 'none',
          background: 'radial-gradient(circle at 50% 50%, transparent 65%, rgba(28, 62, 59, 0.15) 100%)',
          boxShadow: 'inset 0 0 100px rgba(28, 62, 59, 0.12)',
        }}
      />
    </AbsoluteFill>
  );
};

/* --- 1. AMBIENT CONTOURS LAYER --- */
const PestoContoursLayer: React.FC<{currentPanPx: number}> = ({currentPanPx}) => {
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
      {[0, 1, 2].map((k) => (
        <div
          key={k}
          style={{
            width: stride,
            height: 1080,
            position: 'relative',
            overflow: 'hidden',
          }}
        >
          {/* Subtle white contour curves */}
          <div
            style={{
              position: 'absolute',
              top: -120,
              left: 200,
              width: 900,
              height: 900,
              borderRadius: 9999,
              border: '2px solid rgba(255, 255, 255, 0.45)',
            }}
          />
          <div
            style={{
              position: 'absolute',
              top: 250,
              left: 1100,
              width: 820,
              height: 820,
              borderRadius: 9999,
              border: '2px solid rgba(255, 255, 255, 0.38)',
            }}
          />
        </div>
      ))}
    </div>
  );
};

/* --- 2. FAR CANOPY RIDGES LAYER (Parallax 0.2x) --- */
const PestoFarCanopyLayer: React.FC<{currentPanPx: number}> = ({currentPanPx}) => {
  const stride = 2400;
  const offsetX = -(((currentPanPx % stride) + stride) % stride);

  return (
    <div
      style={{
        position: 'absolute',
        bottom: 0,
        left: 0,
        width: stride * 3,
        height: 480,
        transform: `translateX(${offsetX - stride}px)`,
        pointerEvents: 'none',
        display: 'flex',
      }}
    >
      {[0, 1, 2].map((k) => (
        <svg
          key={k}
          width={stride}
          height={480}
          viewBox={`0 0 ${stride} 480`}
          style={{display: 'block'}}
        >
          <path
            d={`
              M 0 480
              L 0 240
              Q 300 160 600 220
              Q 900 280 1200 180
              Q 1500 80 1800 160
              Q 2100 240 2400 190
              L 2400 480
              Z
            `}
            fill="rgba(43, 83, 79, 0.08)"
          />
        </svg>
      ))}
    </div>
  );
};

/* --- 3. MIDGROUND FOREST GROVE LAYER (Parallax 0.5x) --- */
const PestoMidCanopyLayer: React.FC<{currentPanPx: number}> = ({currentPanPx}) => {
  const stride = 2400;
  const offsetX = -(((currentPanPx % stride) + stride) % stride);

  return (
    <div
      style={{
        position: 'absolute',
        bottom: 0,
        left: 0,
        width: stride * 3,
        height: 380,
        transform: `translateX(${offsetX - stride}px)`,
        pointerEvents: 'none',
        display: 'flex',
      }}
    >
      {[0, 1, 2].map((k) => (
        <svg
          key={k}
          width={stride}
          height={380}
          viewBox={`0 0 ${stride} 380`}
          style={{display: 'block'}}
        >
          <path
            d={`
              M 0 380
              L 0 160
              Q 250 80 500 130
              Q 750 180 1000 110
              Q 1300 40 1600 120
              Q 1900 200 2150 140
              Q 2300 100 2400 150
              L 2400 380
              Z
            `}
            fill="rgba(43, 83, 79, 0.14)"
          />
        </svg>
      ))}
    </div>
  );
};

/* --- 4. SLIDE CONTENTS LAYER --- */
const PestoSlideContentsLayer: React.FC<{
  currentClearingIdx: number;
  isPaused: boolean;
  slideProgress: number;
  frame: number;
}> = ({currentClearingIdx, isPaused, slideProgress, frame}) => {
  const isFinalScene = currentClearingIdx === 4;

  // For scenes 0-3, fade out before turn; for scene 4, transition to outro CTA and hold until end
  let opacity = 1;
  if (!isFinalScene) {
    opacity = interpolate(slideProgress, [0, 8, 86, 95], [0, 1, 1, 0], {
      extrapolateLeft: 'clamp',
      extrapolateRight: 'clamp',
    });
  } else {
    // Scene 4 / Outro: stays fully visible through end of video
    opacity = interpolate(frame, [720, 728, 892, 900], [0, 1, 1, 0.8], {
      extrapolateLeft: 'clamp',
      extrapolateRight: 'clamp',
    });
  }

  if (opacity <= 0.001) return null;

  const currentClearing = CLEARINGS[currentClearingIdx] || CLEARINGS[0];

  // In final scene (720-900), frame >= 795 switches to Outro CTA
  const isOutro = isFinalScene && frame >= 795;
  const outroProgress = frame - 795;

  return (
    <AbsoluteFill
      style={{
        opacity,
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'flex-start',
        paddingTop: 50,
      }}
    >
      {/* Title & Subtitle */}
      {!isOutro ? (
        <PestoTitle
          title={currentClearing.title}
          subtitle={currentClearing.subtitle}
          slideProgress={slideProgress}
          eyebrow={
            currentClearingIdx === 0
              ? 'MEET TAILORBIRD // V0.2.0'
              : currentClearingIdx === 1
              ? 'PROBLEM: BURNOUT'
              : currentClearingIdx === 2
              ? 'COCKPIT ARCHITECTURE'
              : currentClearingIdx === 3
              ? 'RAPID ATS SUBMISSION'
              : 'OFFLINE PRIVACY'
          }
        />
      ) : (
        <PestoTitle
          title="GET YOUR JOB SEARCH WINGMAN"
          subtitle="Free & offline desktop app for Windows, macOS, and Linux."
          slideProgress={outroProgress}
          eyebrow="OFFICIAL LAUNCH"
        />
      )}

      {/* Dedicated Animated Showcase */}
      {currentClearingIdx === 0 && <PestoIntroShowcase slideProgress={slideProgress} />}
      {currentClearingIdx === 1 && <PestoChaosShowcase slideProgress={slideProgress} />}
      {currentClearingIdx === 2 && <PestoDualPaneShowcase slideProgress={slideProgress} />}
      {currentClearingIdx === 3 && <PestoAutofillShowcase slideProgress={slideProgress} />}
      {currentClearingIdx === 4 && (
        !isOutro ? (
          <PestoPipelineShowcase slideProgress={slideProgress} />
        ) : (
          <PestoOutroShowcase slideProgress={outroProgress} />
        )
      )}
    </AbsoluteFill>
  );
};

/* --- 5. FOREGROUND SILHOUETTE TREE LAYER --- */
const PestoForegroundTreeLayer: React.FC<{frame: number}> = ({frame}) => {
  const normFrame = ((frame % TOTAL_FRAMES) + TOTAL_FRAMES) % TOTAL_FRAMES;
  const segmentLength = 180;
  const frameInSegment = normFrame % segmentLength;
  const segmentIdx = Math.floor(normFrame / segmentLength);

  // Do NOT sweep tree in segment 4 (hold final outro screen!)
  if (segmentIdx === 4 && normFrame < 892) {
    return null;
  }

  // Tree sweeps across only during the transition phase (frame 86 to 184)
  const sweepStart = 86;
  const sweepEnd = 184;

  if (frameInSegment < sweepStart && frameInSegment > 4) {
    return null;
  }

  // Continuous smooth ease across the screen from right (+2200px) to left (-2200px)
  let t = 0;
  if (frameInSegment >= sweepStart) {
    t = (frameInSegment - sweepStart) / (sweepEnd - sweepStart);
  } else {
    // Wrap around for the first few frames
    t = (frameInSegment + (segmentLength - sweepStart)) / (sweepEnd - sweepStart);
  }

  const smoothT = t * t * (3 - 2 * t);
  const posX = interpolate(smoothT, [0, 1], [2200, -2200]);

  return (
    <div
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: 1400,
        height: 1080,
        transform: `translateX(${posX}px)`,
        pointerEvents: 'none',
        zIndex: 50,
      }}
    >
      <svg
        width={1400}
        height={1080}
        viewBox="0 0 1400 1080"
        fill="none"
        style={{display: 'block'}}
      >
        <defs>
          <linearGradient id="pestoTrunkGrad" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" stopColor="#2b534f" />
            <stop offset="40%" stopColor="#1c3e3b" />
            <stop offset="100%" stopColor="#0f2623" />
          </linearGradient>
        </defs>

        {/* Massive organic silhouette tree trunk */}
        <path
          d="
            M 300 1080
            C 400 900, 320 740, 420 540
            C 480 380, 400 220, 470 60
            C 490 -10, 450 0, 480 0
            L 920 0
            C 880 140, 960 300, 910 480
            C 980 660, 920 860, 990 1020
            C 1020 1060, 980 1080, 1060 1080
            Z
          "
          fill="url(#pestoTrunkGrad)"
        />

        {/* Natural organic branches extending outwards */}
        <path
          d="
            M 420 540
            C 260 500, 140 440, 0 460
            L 0 380
            C 160 380, 310 440, 450 490
            Z
          "
          fill="url(#pestoTrunkGrad)"
        />
        <path
          d="
            M 910 480
            C 1060 430, 1220 370, 1400 390
            L 1400 480
            C 1230 460, 1070 510, 930 550
            Z
          "
          fill="url(#pestoTrunkGrad)"
        />

        {/* Stylized canopy clumps */}
        <circle cx={480} cy={140} r={180} fill="#1c3e3b" />
        <circle cx={340} cy={180} r={140} fill="#234743" />
        <circle cx={920} cy={150} r={190} fill="#1c3e3b" />
        <circle cx={1060} cy={200} r={150} fill="#234743" />
      </svg>
    </div>
  );
};
