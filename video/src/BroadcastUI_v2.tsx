import React from 'react';
import {
  Img,
  interpolate,
  spring,
  staticFile,
  useCurrentFrame,
  useVideoConfig,
} from 'remotion';
import {Pesto, outfitFont, monoFont} from './theme_pesto';

/**
 * Pesto Title - semi-bold 600 Outfit typography matching GitHub Pages
 */
export const PestoTitle: React.FC<{
  title: string;
  subtitle: string;
  slideProgress: number;
  eyebrow?: string;
}> = ({title, subtitle, slideProgress, eyebrow}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress,
    fps,
    config: {damping: 14, stiffness: 140},
  });

  const subEntrance = spring({
    frame: slideProgress - 4,
    fps,
    config: {damping: 15, stiffness: 130},
  });

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        textAlign: 'center',
        marginBottom: 16,
        zIndex: 10,
      }}
    >
      {eyebrow && (
        <div
          style={{
            fontFamily: monoFont,
            fontSize: 13,
            fontWeight: 600,
            textTransform: 'uppercase',
            letterSpacing: 2,
            color: Pesto.textDim,
            marginBottom: 6,
            opacity: entrance,
            transform: `translateY(${interpolate(entrance, [0, 1], [15, 0])}px)`,
          }}
        >
          {eyebrow}
        </div>
      )}

      <div
        style={{
          fontFamily: outfitFont,
          fontWeight: 600,
          fontSize: 60,
          lineHeight: 1.1,
          letterSpacing: -1,
          color: Pesto.textPrimary,
          opacity: entrance,
          transform: `translateY(${interpolate(entrance, [0, 1], [18, 0])}px) scale(${interpolate(entrance, [0, 1], [0.94, 1.0])})`,
          textShadow: '0 4px 24px rgba(28, 62, 59, 0.12)',
        }}
      >
        {title}
      </div>

      <div
        style={{
          fontFamily: outfitFont,
          fontWeight: 500,
          fontSize: 22,
          lineHeight: 1.3,
          color: Pesto.textSecondary,
          opacity: subEntrance,
          transform: `translateY(${interpolate(subEntrance, [0, 1], [14, 0])}px)`,
          marginTop: 6,
          maxWidth: 960,
        }}
      >
        {subtitle}
      </div>
    </div>
  );
};

/**
 * Scene 1: Intro / Brand Overview
 */
export const PestoIntroShowcase: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 5,
    fps,
    config: {damping: 13, stiffness: 120},
  });

  // Gentle mascot floating bob
  const mascotBob = Math.sin(slideProgress / 14) * 8;
  const mascotTilt = Math.sin(slideProgress / 18) * 3;

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        opacity: entrance,
        transform: `translateY(${interpolate(entrance, [0, 1], [30, 0])}px)`,
      }}
    >
      {/* 3D Mascot Centerpiece */}
      <div
        style={{
          position: 'relative',
          width: 240,
          height: 240,
          marginBottom: 20,
          transform: `translateY(${mascotBob}px) rotate(${mascotTilt}deg)`,
        }}
      >
        <Img
          src={staticFile('images/bird-waving-no-bg.png')}
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'contain',
            filter: 'drop-shadow(0 20px 30px rgba(28, 62, 59, 0.18))',
          }}
        />
      </div>

      {/* Pesto Feature Badges Row */}
      <div
        style={{
          display: 'flex',
          gap: 28,
          marginBottom: 32,
        }}
      >
        {[
          {icon: '🛡️', text: '100% Offline & Private'},
          {icon: '⚡', text: 'Native Desktop Speed'},
          {icon: '💚', text: 'Free & Open Source'},
        ].map((badge, i) => {
          const badgeEntrance = spring({
            frame: slideProgress - 8 - i * 3,
            fps,
            config: {damping: 12, stiffness: 140},
          });

          return (
            <div
              key={badge.text}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 10,
                padding: '10px 20px',
                borderRadius: 999,
                border: `1.5px solid ${Pesto.borderMedium}`,
                background: 'rgba(255, 255, 255, 0.4)',
                backdropFilter: 'blur(8px)',
                fontFamily: outfitFont,
                fontSize: 18,
                fontWeight: 600,
                color: Pesto.textPrimary,
                opacity: badgeEntrance,
                transform: `scale(${interpolate(badgeEntrance, [0, 1], [0.85, 1])})`,
                boxShadow: Pesto.shadowLight,
              }}
            >
              <span>{badge.icon}</span>
              <span>{badge.text}</span>
            </div>
          );
        })}
      </div>

      {/* Minimal OS Download Buttons matching docs/index.html */}
      <div
        style={{
          display: 'flex',
          gap: 18,
        }}
      >
        {[
          {name: 'Windows', icon: '🪟', recommended: true},
          {name: 'macOS', icon: '🍎', recommended: false},
          {name: 'Linux', icon: '🐧', recommended: false},
        ].map((os, i) => (
          <div
            key={os.name}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 14,
              padding: '12px 28px',
              borderRadius: 999,
              border: os.recommended ? `2px solid ${Pesto.textPrimary}` : `1.5px solid ${Pesto.borderMedium}`,
              background: 'transparent',
              fontFamily: outfitFont,
              fontSize: 18,
              fontWeight: 600,
              color: Pesto.textPrimary,
              boxShadow: '0 4px 15px rgba(28, 62, 59, 0.08)',
            }}
          >
            <span>{os.icon}</span>
            <span>{os.name}</span>
            <span style={{opacity: 0.6}}>↓</span>
          </div>
        ))}
      </div>
    </div>
  );
};

/**
 * Scene 2: Tab Chaos vs Parallel Focus
 */
export const PestoChaosShowcase: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 4,
    fps,
    config: {damping: 14, stiffness: 120},
  });

  const tabList = [
    {title: 'Greenhouse — Senior Nest Architect', url: 'boards.greenhouse.io/wren', jitter: -12},
    {title: 'Lever Application — Leaf Stitcher', url: 'jobs.lever.co/finch', jitter: 14},
    {title: 'Ashby — Flight Dispatcher Lead', url: 'jobs.ashbyhq.com/falcon', jitter: -6},
    {title: 'Workday Portal — Canopy Ops', url: 'myworkdayjobs.com/peckwell', jitter: 10},
    {title: 'Job Hunting Tracker 2026.xlsx', url: 'docs.google.com/spreadsheets', jitter: -18},
    {title: 'Resume_Final_v14_REAL_FINAL.pdf', url: 'file:///C:/Users/Downloads', jitter: 8},
  ];

  return (
    <div
      style={{
        position: 'relative',
        width: 1040,
        height: 520,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        opacity: entrance,
      }}
    >
      {tabList.map((tab, i) => {
        const tabEntrance = spring({
          frame: slideProgress - 3 - i * 3,
          fps,
          config: {damping: 12, stiffness: 130},
        });

        const floatY = Math.sin((slideProgress + i * 20) / 12) * 8;
        const rot = tab.jitter + Math.cos((slideProgress + i * 15) / 15) * 2;

        return (
          <div
            key={tab.title}
            style={{
              position: 'absolute',
              width: 580,
              padding: '16px 24px',
              borderRadius: 14,
              border: `1.5px solid ${Pesto.borderMedium}`,
              background: 'rgba(255, 255, 255, 0.75)',
              backdropFilter: 'blur(12px)',
              boxShadow: '0 20px 45px rgba(28, 62, 59, 0.16)',
              transform: `translateY(${floatY + (i - 2.5) * 35}px) translateX(${(i - 2.5) * 45}px) rotate(${rot}deg) scale(${interpolate(tabEntrance, [0, 1], [0.8, 1])})`,
              opacity: tabEntrance,
              zIndex: 10 + i,
            }}
          >
            <div style={{display: 'flex', alignItems: 'center', gap: 10, marginBottom: 6}}>
              <div style={{width: 10, height: 10, borderRadius: 5, background: '#ef4444'}} />
              <div style={{width: 10, height: 10, borderRadius: 5, background: '#f59e0b'}} />
              <div style={{width: 10, height: 10, borderRadius: 5, background: '#10b981'}} />
              <span style={{fontFamily: monoFont, fontSize: 13, color: Pesto.textMuted}}>
                tab {i + 1} of 37
              </span>
            </div>
            <div style={{fontFamily: outfitFont, fontSize: 18, fontWeight: 600, color: Pesto.textPrimary}}>
              {tab.title}
            </div>
            <div style={{fontFamily: monoFont, fontSize: 13, color: Pesto.textDim, marginTop: 4}}>
              {tab.url}
            </div>
          </div>
        );
      })}

      {/* Resolution Badge */}
      <div
        style={{
          position: 'absolute',
          bottom: 20,
          padding: '14px 32px',
          borderRadius: 999,
          background: Pesto.textPrimary,
          color: '#ffffff',
          fontFamily: outfitFont,
          fontSize: 20,
          fontWeight: 600,
          letterSpacing: 0.5,
          boxShadow: Pesto.shadowHeavy,
          zIndex: 40,
          transform: `scale(${interpolate(
            spring({frame: slideProgress - 25, fps, config: {damping: 12, stiffness: 150}}),
            [0, 1],
            [0.7, 1]
          )})`,
          opacity: spring({frame: slideProgress - 25, fps}),
        }}
      >
        ✨ Stop the tab sprawl. Consolidate into Tailorbird.
      </div>
    </div>
  );
};

/**
 * Scene 3: Dual-Pane Workspace (Real Screenshot Animated in 2.5D)
 */
export const PestoDualPaneShowcase: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 5,
    fps,
    config: {damping: 14, stiffness: 120},
  });

  const driftY = Math.sin(slideProgress / 16) * 5;
  const driftRotate = Math.cos(slideProgress / 20) * 0.6;

  return (
    <div
      style={{
        position: 'relative',
        width: 1100,
        height: 520,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      {/* 2.5D Window Container */}
      <div
        style={{
          position: 'relative',
          width: 960,
          height: 490,
          borderRadius: 16,
          border: `2px solid ${Pesto.borderMedium}`,
          background: 'rgba(255, 255, 255, 0.45)',
          backdropFilter: 'blur(16px)',
          boxShadow: Pesto.shadowHeavy,
          overflow: 'hidden',
          display: 'flex',
          flexDirection: 'column',
          transform: `perspective(1200px) rotateY(-4deg) rotateX(2.5deg) rotateZ(${driftRotate}deg) translateY(${driftY}px) scale(${interpolate(entrance, [0, 1], [0.88, 1])})`,
          opacity: entrance,
        }}
      >
        {/* Window Chrome Bar */}
        <div
          style={{
            height: 36,
            flexShrink: 0,
            borderBottom: `1px solid ${Pesto.borderSubtle}`,
            background: 'rgba(255, 255, 255, 0.65)',
            display: 'flex',
            alignItems: 'center',
            padding: '0 16px',
            gap: 8,
          }}
        >
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotClose}} />
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotMin}} />
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotMax}} />
          <div
            style={{
              marginLeft: 14,
              fontFamily: monoFont,
              fontSize: 12,
              fontWeight: 600,
              color: Pesto.textMuted,
            }}
          >
            Tailorbird — Dual-Pane Cockpit
          </div>
        </div>

        {/* Real Screenshot */}
        <div style={{flex: 1, width: '100%', overflow: 'hidden', position: 'relative'}}>
          <Img
            src={staticFile('images/dual-pane-workspace.png')}
            style={{width: '100%', height: '100%', objectFit: 'cover', objectPosition: 'top'}}
          />
        </div>
      </div>

      {/* Mascot Looking Down Peeking from Top Right */}
      <div
        style={{
          position: 'absolute',
          top: -20,
          right: 35,
          width: 180,
          height: 180,
          zIndex: 30,
          pointerEvents: 'none',
          transform: `translateY(${Math.sin(slideProgress / 12) * 5}px)`,
          opacity: entrance,
        }}
      >
        <Img
          src={staticFile('images/bird-looking-down-no-bg.png')}
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'contain',
            filter: 'drop-shadow(0 15px 25px rgba(28, 62, 59, 0.2))',
          }}
        />
      </div>

      {/* Floating Feature Annotation Pills */}
      <div
        style={{
          position: 'absolute',
          bottom: 15,
          left: 40,
          padding: '10px 22px',
          borderRadius: 999,
          background: 'rgba(255, 255, 255, 0.85)',
          border: `1.5px solid ${Pesto.borderMedium}`,
          boxShadow: Pesto.shadowLight,
          fontFamily: outfitFont,
          fontSize: 16,
          fontWeight: 600,
          color: Pesto.textPrimary,
          zIndex: 25,
          transform: `translateY(${interpolate(
            spring({frame: slideProgress - 15, fps, config: {damping: 12, stiffness: 140}}),
            [0, 1],
            [20, 0]
          )}px)`,
          opacity: spring({frame: slideProgress - 15, fps}),
        }}
      >
        📌 Pinned Candidate Profile & Shortcuts
      </div>

      <div
        style={{
          position: 'absolute',
          bottom: 15,
          right: 40,
          padding: '10px 22px',
          borderRadius: 999,
          background: 'rgba(255, 255, 255, 0.85)',
          border: `1.5px solid ${Pesto.borderMedium}`,
          boxShadow: Pesto.shadowLight,
          fontFamily: outfitFont,
          fontSize: 16,
          fontWeight: 600,
          color: Pesto.textPrimary,
          zIndex: 25,
          transform: `translateY(${interpolate(
            spring({frame: slideProgress - 20, fps, config: {damping: 12, stiffness: 140}}),
            [0, 1],
            [20, 0]
          )}px)`,
          opacity: spring({frame: slideProgress - 20, fps}),
        }}
      >
        🌐 Live Chromium Application Form Pane
      </div>
    </div>
  );
};

/**
 * Scene 4: 1-Click Application Autofill (Real Screenshot + Live Animation)
 */
export const PestoAutofillShowcase: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 4,
    fps,
    config: {damping: 14, stiffness: 120},
  });

  const clickFrame = 28;
  const isClicked = slideProgress >= clickFrame;

  // Cursor animation
  const cursorX = interpolate(slideProgress, [0, clickFrame, clickFrame + 8], [800, 460, 460], {
    extrapolateRight: 'clamp',
  });
  const cursorY = interpolate(slideProgress, [0, clickFrame, clickFrame + 8], [400, 230, 230], {
    extrapolateRight: 'clamp',
  });
  const cursorClickScale = slideProgress >= clickFrame && slideProgress <= clickFrame + 6 ? 0.85 : 1;

  // Success Celebration Spring
  const successSpring = spring({
    frame: slideProgress - clickFrame,
    fps,
    config: {damping: 12, stiffness: 160},
  });

  return (
    <div
      style={{
        position: 'relative',
        width: 1100,
        height: 520,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      {/* 2.5D Window Container */}
      <div
        style={{
          position: 'relative',
          width: 960,
          height: 490,
          borderRadius: 16,
          border: isClicked ? `2px solid ${Pesto.accentEmerald}` : `2px solid ${Pesto.borderMedium}`,
          background: 'rgba(255, 255, 255, 0.45)',
          backdropFilter: 'blur(16px)',
          boxShadow: isClicked
            ? `0 35px 90px -20px rgba(16, 185, 129, 0.25)`
            : Pesto.shadowHeavy,
          overflow: 'hidden',
          display: 'flex',
          flexDirection: 'column',
          transform: `perspective(1200px) rotateY(3deg) rotateX(2deg) scale(${interpolate(entrance, [0, 1], [0.88, 1])})`,
          opacity: entrance,
          transition: 'border 0.3s ease, box-shadow 0.3s ease',
        }}
      >
        {/* Window Chrome Bar */}
        <div
          style={{
            height: 36,
            flexShrink: 0,
            borderBottom: `1px solid ${Pesto.borderSubtle}`,
            background: 'rgba(255, 255, 255, 0.65)',
            display: 'flex',
            alignItems: 'center',
            padding: '0 16px',
            gap: 8,
          }}
        >
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotClose}} />
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotMin}} />
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotMax}} />
          <div
            style={{
              marginLeft: 14,
              fontFamily: monoFont,
              fontSize: 12,
              fontWeight: 600,
              color: Pesto.textMuted,
            }}
          >
            Greenhouse ATS — Smart Application Autofill
          </div>
        </div>

        {/* Real Screenshot */}
        <div style={{flex: 1, width: '100%', overflow: 'hidden', position: 'relative'}}>
          <Img
            src={staticFile('images/one-click-auto-fill.png')}
            style={{width: '100%', height: '100%', objectFit: 'cover', objectPosition: 'top'}}
          />

          {/* Green highlight ripple overlay when autofilled */}
          {isClicked && (
            <div
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                width: '100%',
                height: '100%',
                background: 'rgba(16, 185, 129, 0.08)',
                pointerEvents: 'none',
                opacity: interpolate(slideProgress, [clickFrame, clickFrame + 25], [0.6, 0.15], {
                  extrapolateRight: 'clamp',
                }),
              }}
            />
          )}
        </div>
      </div>

      {/* Mascot Running Across Top Left */}
      <div
        style={{
          position: 'absolute',
          top: -30,
          left: 30,
          width: 185,
          height: 185,
          zIndex: 30,
          pointerEvents: 'none',
          transform: `translateY(${Math.sin(slideProgress / 8) * 4}px)`,
          opacity: entrance,
        }}
      >
        <Img
          src={staticFile('images/bird-running-no-bg.png')}
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'contain',
            filter: 'drop-shadow(0 15px 25px rgba(28, 62, 59, 0.2))',
          }}
        />
      </div>

      {/* Animated Mouse Cursor */}
      {slideProgress < clickFrame + 20 && (
        <div
          style={{
            position: 'absolute',
            left: cursorX,
            top: cursorY,
            width: 30,
            height: 30,
            zIndex: 40,
            pointerEvents: 'none',
            transform: `scale(${cursorClickScale})`,
            filter: 'drop-shadow(0 4px 10px rgba(0, 0, 0, 0.3))',
          }}
        >
          <svg width="28" height="28" viewBox="0 0 24 24" fill={Pesto.textPrimary} stroke="#ffffff" strokeWidth="2">
            <path d="M3 3l7 18 3-7 7-3L3 3z" />
          </svg>
        </div>
      )}

      {/* Celebratory Success Badge */}
      {isClicked && (
        <div
          style={{
            position: 'absolute',
            bottom: 30,
            padding: '14px 34px',
            borderRadius: 999,
            background: Pesto.accentEmerald,
            color: '#ffffff',
            fontFamily: outfitFont,
            fontSize: 22,
            fontWeight: 700,
            boxShadow: '0 20px 50px rgba(16, 185, 129, 0.4)',
            zIndex: 50,
            transform: `scale(${interpolate(successSpring, [0, 1], [0.5, 1])})`,
            opacity: successSpring,
            display: 'flex',
            alignItems: 'center',
            gap: 12,
          }}
        >
          <span>✓</span>
          <span>27 Fields Autofilled in 0.3 Seconds!</span>
        </div>
      )}
    </div>
  );
};

/**
 * Scene 5: Prospects Pipeline & 100% Local Privacy (Real Screenshot)
 */
export const PestoPipelineShowcase: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 5,
    fps,
    config: {damping: 14, stiffness: 120},
  });

  const driftY = Math.sin(slideProgress / 18) * 5;

  return (
    <div
      style={{
        position: 'relative',
        width: 1100,
        height: 520,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      {/* 2.5D Window Container */}
      <div
        style={{
          position: 'relative',
          width: 960,
          height: 490,
          borderRadius: 16,
          border: `2px solid ${Pesto.borderMedium}`,
          background: 'rgba(255, 255, 255, 0.45)',
          backdropFilter: 'blur(16px)',
          boxShadow: Pesto.shadowHeavy,
          overflow: 'hidden',
          display: 'flex',
          flexDirection: 'column',
          transform: `perspective(1200px) rotateY(-3deg) rotateX(2deg) translateY(${driftY}px) scale(${interpolate(entrance, [0, 1], [0.88, 1])})`,
          opacity: entrance,
        }}
      >
        {/* Window Chrome Bar */}
        <div
          style={{
            height: 36,
            flexShrink: 0,
            borderBottom: `1px solid ${Pesto.borderSubtle}`,
            background: 'rgba(255, 255, 255, 0.65)',
            display: 'flex',
            alignItems: 'center',
            padding: '0 16px',
            gap: 8,
          }}
        >
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotClose}} />
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotMin}} />
          <div style={{width: 11, height: 11, borderRadius: 999, background: Pesto.dotMax}} />
          <div
            style={{
              marginLeft: 14,
              fontFamily: monoFont,
              fontSize: 12,
              fontWeight: 600,
              color: Pesto.textMuted,
            }}
          >
            Tailorbird — Prospects Pipeline & Tracking
          </div>
        </div>

        {/* Real Screenshot */}
        <div style={{flex: 1, width: '100%', overflow: 'hidden', position: 'relative'}}>
          <Img
            src={staticFile('images/prospects-pipeline.png')}
            style={{width: '100%', height: '100%', objectFit: 'cover', objectPosition: 'top'}}
          />
        </div>
      </div>

      {/* Mascot Head Tilt Peeking from Top Right */}
      <div
        style={{
          position: 'absolute',
          top: -25,
          right: 35,
          width: 180,
          height: 180,
          zIndex: 30,
          pointerEvents: 'none',
          transform: `translateY(${Math.sin(slideProgress / 14) * 5}px)`,
          opacity: entrance,
        }}
      >
        <Img
          src={staticFile('images/bird-head-tilt-no-bg.png')}
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'contain',
            filter: 'drop-shadow(0 15px 25px rgba(28, 62, 59, 0.2))',
          }}
        />
      </div>

      {/* Stage Progression Pills Badge */}
      <div
        style={{
          position: 'absolute',
          bottom: 20,
          display: 'flex',
          gap: 12,
          padding: '12px 24px',
          borderRadius: 999,
          background: 'rgba(255, 255, 255, 0.9)',
          border: `1.5px solid ${Pesto.borderMedium}`,
          boxShadow: Pesto.shadowLight,
          zIndex: 40,
          opacity: spring({frame: slideProgress - 12, fps}),
        }}
      >
        {['Bookmarked', 'Applied', 'Screening', 'Interviewing', 'Offer'].map((stage, i) => (
          <div
            key={stage}
            style={{
              padding: '6px 14px',
              borderRadius: 999,
              fontFamily: outfitFont,
              fontSize: 15,
              fontWeight: 600,
              background: i === 3 ? Pesto.textPrimary : 'transparent',
              color: i === 3 ? '#ffffff' : Pesto.textSecondary,
              border: i === 3 ? 'none' : `1px solid ${Pesto.borderSubtle}`,
            }}
          >
            {stage}
          </div>
        ))}
      </div>
    </div>
  );
};

/**
 * Scene 6 / Outro: Call to Action & Download Options
 */
export const PestoOutroShowcase: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 4,
    fps,
    config: {damping: 14, stiffness: 130},
  });

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        opacity: entrance,
        transform: `scale(${interpolate(entrance, [0, 1], [0.92, 1])})`,
      }}
    >
      {/* Singing Mascot */}
      <div
        style={{
          position: 'relative',
          width: 220,
          height: 220,
          marginBottom: 16,
          transform: `translateY(${Math.sin(slideProgress / 10) * 6}px)`,
        }}
      >
        <Img
          src={staticFile('images/bird-singing-no-bg.png')}
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'contain',
            filter: 'drop-shadow(0 20px 35px rgba(28, 62, 59, 0.2))',
          }}
        />

        {/* Floating Musical Notes */}
        {[0, 1, 2].map((note, i) => {
          const noteY = (slideProgress * 2.5 + i * 35) % 90;
          const noteX = Math.sin((slideProgress + i * 20) / 10) * 25;
          const noteOpacity = Math.max(0, 1 - noteY / 85);

          return (
            <div
              key={note}
              style={{
                position: 'absolute',
                top: 40 - noteY,
                right: 20 + noteX,
                fontSize: 26,
                color: Pesto.textPrimary,
                opacity: noteOpacity,
              }}
            >
              🎵
            </div>
          );
        })}
      </div>

      {/* Download Outline Buttons (Windows, macOS, Linux) */}
      <div
        style={{
          display: 'flex',
          gap: 20,
          marginBottom: 24,
        }}
      >
        {[
          {name: 'Windows', icon: '🪟', recommended: true},
          {name: 'macOS', icon: '🍎', recommended: false},
          {name: 'Linux', icon: '🐧', recommended: false},
        ].map((os) => (
          <div
            key={os.name}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 14,
              padding: '14px 34px',
              borderRadius: 999,
              border: os.recommended ? `2.5px solid ${Pesto.textPrimary}` : `1.5px solid ${Pesto.borderMedium}`,
              background: 'transparent',
              fontFamily: outfitFont,
              fontSize: 20,
              fontWeight: 600,
              color: Pesto.textPrimary,
              boxShadow: '0 6px 20px rgba(28, 62, 59, 0.1)',
            }}
          >
            <span>{os.icon}</span>
            <span>{os.name}</span>
            <span style={{opacity: 0.6}}>↓</span>
          </div>
        ))}
      </div>

      {/* URL Link Badge */}
      <div
        style={{
          padding: '10px 28px',
          borderRadius: 999,
          border: `1.5px solid ${Pesto.borderSubtle}`,
          background: 'rgba(255, 255, 255, 0.5)',
          fontFamily: monoFont,
          fontSize: 16,
          fontWeight: 600,
          color: Pesto.textPrimary,
        }}
      >
        joelstransky.github.io/Tailorbird
      </div>
    </div>
  );
};
