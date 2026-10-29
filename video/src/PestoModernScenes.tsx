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
import {VrSpatialContainer} from './VrSpatialContainer';

/**
 * =========================================================================
 * SCENE 1: HOOK & MEET TAILORBIRD (Frames 0 - 355)
 * Transition: Enter Center -> Exit to LEFT
 * Gap: 355 - 370 (15 frames clean virtual gap)
 * Subheading doubled in size, generous reading time for both phases & bullets
 * =========================================================================
 */
export const ModernScene1Hook: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();

  const isSolution = frame >= 140;

  // Staggered springs for Problem/Inquiry phase (Generous 140-frame hold)
  const probTitle = spring({frame: frame - 6, fps, config: {damping: 14, stiffness: 75, mass: 1.15}});
  const probSub = spring({frame: frame - 18, fps, config: {damping: 14, stiffness: 80}});

  // Staggered springs for Solution phase (Frames 140 - 330)
  const solMascot = spring({frame: frame - 140, fps, config: {damping: 14, stiffness: 75, mass: 1.15}});
  const solTitle = spring({frame: frame - 152, fps, config: {damping: 14, stiffness: 80, mass: 1.1}});
  const solSub = spring({frame: frame - 165, fps, config: {damping: 14, stiffness: 80}});

  // Bullets come in one by one with comfortable reading pacing
  const bullet1 = spring({frame: frame - 188, fps, config: {damping: 14, stiffness: 85}});
  const bullet2 = spring({frame: frame - 216, fps, config: {damping: 14, stiffness: 85}});
  const bullet3 = spring({frame: frame - 244, fps, config: {damping: 14, stiffness: 85}});

  const mascotBob = Math.sin(frame / 14) * 8;
  const mascotRotate = Math.sin(frame / 18) * 3;

  return (
    <VrSpatialContainer
      enterStart={0}
      enterDuration={24}
      exitStart={330}
      exitDuration={25}
      enterDirection="from-center"
      exitDirection="to-left"
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        {!isSolution ? (
          // Hook Part A: The Empathetic Question (Subheading doubled in font size)
          <div style={{display: 'flex', flexDirection: 'column', alignItems: 'center', textAlign: 'center'}}>
            <div
              style={{
                fontFamily: outfitFont,
                fontSize: 68,
                fontWeight: 600,
                color: Pesto.textPrimary,
                letterSpacing: -1.6,
                lineHeight: 1.15,
                maxWidth: 1160,
                opacity: probTitle,
                transform: `translateY(${interpolate(probTitle, [0, 1], [30, 0])}px) scale(${interpolate(probTitle, [0, 1], [0.94, 1])})`,
              }}
            >
              What if there was a tool to actually help with the job search?
            </div>

            <div
              style={{
                fontFamily: outfitFont,
                fontSize: 44,
                color: Pesto.textSecondary,
                fontWeight: 450,
                marginTop: 26,
                maxWidth: 1060,
                lineHeight: 1.35,
                opacity: probSub,
                transform: `translateY(${interpolate(probSub, [0, 1], [20, 0])}px)`,
              }}
            >
              A dedicated application cockpit to keep your profile, notes, and research in one calm workspace.
            </div>
          </div>
        ) : (
          // Hook Part B: The Sincere Solution ("Finally, there's a tool" with doubled subheading)
          <div style={{display: 'flex', flexDirection: 'column', alignItems: 'center', textAlign: 'center'}}>
            <div
              style={{
                position: 'relative',
                width: 210,
                height: 210,
                marginBottom: 10,
                transform: `translateY(${mascotBob}px) rotate(${mascotRotate}deg) scale(${interpolate(solMascot, [0, 1], [0.65, 1])})`,
                opacity: solMascot,
              }}
            >
              <Img
                src={staticFile('images/bird-waving-no-bg.png')}
                style={{
                  width: '100%',
                  height: '100%',
                  objectFit: 'contain',
                  filter: 'drop-shadow(0 20px 35px rgba(28, 62, 59, 0.22))',
                }}
              />
            </div>

            <div
              style={{
                fontFamily: outfitFont,
                fontSize: 72,
                fontWeight: 600,
                color: Pesto.textPrimary,
                letterSpacing: -2,
                lineHeight: 1.05,
                opacity: solTitle,
                transform: `translateY(${interpolate(solTitle, [0, 1], [25, 0])}px)`,
              }}
            >
              Finally, there's a tool.
            </div>

            <div
              style={{
                fontFamily: outfitFont,
                fontSize: 44,
                color: Pesto.textSecondary,
                fontWeight: 450,
                marginTop: 14,
                maxWidth: 1100,
                lineHeight: 1.3,
                opacity: solSub,
                transform: `translateY(${interpolate(solSub, [0, 1], [15, 0])}px)`,
              }}
            >
              Meet Tailorbird — the free, private browser built for the applicant.
            </div>

            {/* Bullets coming in one by one with normalized spacing */}
            <div
              style={{
                display: 'flex',
                gap: 40,
                marginTop: 38,
              }}
            >
              {/* Bullet 1 */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 10,
                  fontFamily: outfitFont,
                  fontSize: 22,
                  fontWeight: 600,
                  color: Pesto.textPrimary,
                  opacity: bullet1,
                  transform: `translateY(${interpolate(bullet1, [0, 1], [16, 0])}px)`,
                }}
              >
                <span style={{fontSize: 24}}>🛡️</span>
                <span>100% Offline &amp; Private</span>
              </div>

              {/* Bullet 2 */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 10,
                  fontFamily: outfitFont,
                  fontSize: 22,
                  fontWeight: 600,
                  color: Pesto.textPrimary,
                  opacity: bullet2,
                  transform: `translateY(${interpolate(bullet2, [0, 1], [16, 0])}px)`,
                }}
              >
                <span style={{fontSize: 24}}>⚡</span>
                <span>Native Desktop Speed</span>
              </div>

              {/* Bullet 3 */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 10,
                  fontFamily: outfitFont,
                  fontSize: 22,
                  fontWeight: 600,
                  color: Pesto.textPrimary,
                  opacity: bullet3,
                  transform: `translateY(${interpolate(bullet3, [0, 1], [16, 0])}px)`,
                }}
              >
                <span style={{fontSize: 24}}>🤖</span>
                <span>Model Context Protocol (MCP)</span>
              </div>
            </div>
          </div>
        )}
      </div>
    </VrSpatialContainer>
  );
};

/**
 * =========================================================================
 * SCENE 2: DUAL-PANE COCKPIT (Frames 370 - 565)
 * Transition: Enter from RIGHT -> Exit to UP
 * Gap: 565 - 580 (15 frames clean virtual gap)
 * Pure: Title + Subtitle (Doubled font size) + Screenshot (No eyebrows, no bottom asides)
 * =========================================================================
 */
export const ModernScene2DualPane: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();

  const localFrame = frame - 370;

  // Staggered element springs (slightly slower, smoother)
  const staggerTitle = spring({frame: localFrame - 6, fps, config: {damping: 15, stiffness: 70, mass: 1.15}});
  const staggerSubtitle = spring({frame: localFrame - 18, fps, config: {damping: 15, stiffness: 75}});
  const staggerWindow = spring({frame: localFrame - 30, fps, config: {damping: 14, stiffness: 65, mass: 1.2}});
  const staggerMascot = spring({frame: localFrame - 46, fps, config: {damping: 15, stiffness: 85}});

  // 2.5D perspective tilt & subtle drift
  const rotY = interpolate(localFrame, [0, 195], [-4.0, -1.0]);
  const rotX = interpolate(localFrame, [0, 195], [2.5, 1.0]);
  const driftY = Math.sin(localFrame / 22) * 4;

  return (
    <VrSpatialContainer
      enterStart={370}
      enterDuration={34}
      exitStart={540}
      exitDuration={25}
      enterDirection="from-right"
      exitDirection="to-up"
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        {/* Header Block: Title + Subtitle (Doubled font size, wraps cleanly) */}
        <div style={{textAlign: 'center', marginBottom: 24}}>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 60,
              fontWeight: 600,
              color: Pesto.textPrimary,
              letterSpacing: -1.4,
              lineHeight: 1.08,
              opacity: staggerTitle,
              transform: `translateY(${interpolate(staggerTitle, [0, 1], [20, 0])}px)`,
            }}
          >
            DUAL-PANE WORKSPACE
          </div>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 42,
              color: Pesto.textSecondary,
              fontWeight: 450,
              marginTop: 12,
              maxWidth: 1100,
              lineHeight: 1.3,
              opacity: staggerSubtitle,
              transform: `translateY(${interpolate(staggerSubtitle, [0, 1], [15, 0])}px)`,
            }}
          >
            Pinned candidate cockpit locked alongside a dedicated Chromium browser pane.
          </div>
        </div>

        {/* 2.5D Window Stage */}
        <div
          style={{
            position: 'relative',
            width: 1000,
            height: 480,
            opacity: staggerWindow,
            transform: `perspective(1200px) rotateY(${rotY}deg) rotateX(${rotX}deg) translateY(${driftY}px) scale(${interpolate(staggerWindow, [0, 1], [0.92, 1])})`,
          }}
        >
          {/* Window Chrome & Screenshot (Clipped) */}
          <div
            style={{
              position: 'relative',
              width: '100%',
              height: '100%',
              borderRadius: 16,
              border: `1.5px solid ${Pesto.borderMedium}`,
              background: 'rgba(255, 255, 255, 0.55)',
              backdropFilter: 'blur(20px)',
              boxShadow: Pesto.shadowHeavy,
              overflow: 'hidden',
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            {/* Window Chrome */}
            <div
              style={{
                height: 38,
                flexShrink: 0,
                borderBottom: `1px solid ${Pesto.borderSubtle}`,
                background: 'rgba(255, 255, 255, 0.8)',
                display: 'flex',
                alignItems: 'center',
                padding: '0 18px',
                gap: 8,
              }}
            >
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotClose}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMin}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMax}} />
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

          {/* Mascot Looking Down (Twice size: 280x280) Perched on Top Right */}
          <div
            style={{
              position: 'absolute',
              top: -85,
              right: -35,
              width: 280,
              height: 280,
              zIndex: 35,
              pointerEvents: 'none',
              opacity: staggerMascot,
              transform: `translateY(${Math.sin(localFrame / 16) * 6}px) scale(${interpolate(staggerMascot, [0, 1], [0.75, 1])})`,
            }}
          >
            <Img
              src={staticFile('images/bird-looking-down-no-bg.png')}
              style={{
                width: '100%',
                height: '100%',
                objectFit: 'contain',
                filter: 'drop-shadow(0 20px 35px rgba(28, 62, 59, 0.32))',
              }}
            />
          </div>
        </div>
      </div>
    </VrSpatialContainer>
  );
};

/**
 * =========================================================================
 * SCENE 3: 1-CLICK APPLICATION AUTOFILL (Frames 580 - 775)
 * Transition: Enter from BOTTOM -> Exit to LEFT
 * Gap: 775 - 790 (15 frames clean virtual gap)
 * Pure: Title + Subtitle (Doubled font size) + Screenshot (No eyebrows, no bottom asides)
 * =========================================================================
 */
export const ModernScene3Autofill: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();

  const localFrame = frame - 580;

  // Staggered springs (slightly slower, smoother)
  const staggerTitle = spring({frame: localFrame - 6, fps, config: {damping: 15, stiffness: 70, mass: 1.15}});
  const staggerSubtitle = spring({frame: localFrame - 18, fps, config: {damping: 15, stiffness: 75}});
  const staggerWindow = spring({frame: localFrame - 30, fps, config: {damping: 14, stiffness: 65, mass: 1.2}});
  const staggerMascot = spring({frame: localFrame - 46, fps, config: {damping: 15, stiffness: 85}});

  const clickFrame = 65;
  const isClicked = localFrame >= clickFrame;

  // Cursor glide
  const cursorX = interpolate(localFrame, [16, clickFrame, clickFrame + 8], [940, 480, 480], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });
  const cursorY = interpolate(localFrame, [16, clickFrame, clickFrame + 8], [440, 260, 260], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });
  const cursorScale = localFrame >= clickFrame && localFrame <= clickFrame + 6 ? 0.8 : 1;

  return (
    <VrSpatialContainer
      enterStart={580}
      enterDuration={34}
      exitStart={750}
      exitDuration={25}
      enterDirection="from-bottom"
      exitDirection="to-left"
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        {/* Title Header: Title + Subtitle (Doubled font size, wraps cleanly) */}
        <div style={{textAlign: 'center', marginBottom: 24}}>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 60,
              fontWeight: 600,
              color: Pesto.textPrimary,
              letterSpacing: -1.4,
              lineHeight: 1.08,
              opacity: staggerTitle,
              transform: `translateY(${interpolate(staggerTitle, [0, 1], [20, 0])}px)`,
            }}
          >
            1-CLICK AUTOFILL
          </div>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 42,
              color: Pesto.textSecondary,
              fontWeight: 450,
              marginTop: 12,
              maxWidth: 1100,
              lineHeight: 1.3,
              opacity: staggerSubtitle,
              transform: `translateY(${interpolate(staggerSubtitle, [0, 1], [15, 0])}px)`,
            }}
          >
            Detects and fills fields across Greenhouse, Lever, Ashby, Workday, and more.
          </div>
        </div>

        {/* 2.5D Window Stage */}
        <div
          style={{
            position: 'relative',
            width: 1000,
            height: 480,
            opacity: staggerWindow,
            transform: `perspective(1200px) rotateY(-2deg) rotateX(2deg) scale(${interpolate(staggerWindow, [0, 1], [0.92, 1])})`,
          }}
        >
          {/* Window Chrome & Screenshot (Clipped) */}
          <div
            style={{
              position: 'relative',
              width: '100%',
              height: '100%',
              borderRadius: 16,
              border: `1.5px solid ${isClicked ? Pesto.accentEmerald : Pesto.borderMedium}`,
              background: 'rgba(255, 255, 255, 0.55)',
              backdropFilter: 'blur(20px)',
              boxShadow: isClicked ? '0 25px 65px rgba(16, 185, 129, 0.28)' : Pesto.shadowHeavy,
              overflow: 'hidden',
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            {/* Window Chrome */}
            <div
              style={{
                height: 38,
                flexShrink: 0,
                borderBottom: `1px solid ${Pesto.borderSubtle}`,
                background: 'rgba(255, 255, 255, 0.8)',
                display: 'flex',
                alignItems: 'center',
                padding: '0 18px',
                gap: 8,
              }}
            >
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotClose}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMin}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMax}} />
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

              {/* Click Shockwave Ripple */}
              {isClicked && localFrame < clickFrame + 28 && (
                <div
                  style={{
                    position: 'absolute',
                    left: 480,
                    top: 260,
                    width: (localFrame - clickFrame) * 18,
                    height: (localFrame - clickFrame) * 18,
                    borderRadius: 999,
                    border: '3px solid rgba(16, 185, 129, 0.85)',
                    transform: 'translate(-50%, -50%)',
                    opacity: 1 - (localFrame - clickFrame) / 28,
                    pointerEvents: 'none',
                  }}
                />
              )}
            </div>
          </div>

          {/* Running Mascot (Twice size: 260x260) on Top Left */}
          <div
            style={{
              position: 'absolute',
              top: -80,
              left: -35,
              width: 260,
              height: 260,
              zIndex: 35,
              pointerEvents: 'none',
              opacity: staggerMascot,
              transform: `translateY(${Math.sin(localFrame / 10) * 6}px)`,
            }}
          >
            <Img
              src={staticFile('images/bird-running-no-bg.png')}
              style={{
                width: '100%',
                height: '100%',
                objectFit: 'contain',
                filter: 'drop-shadow(0 20px 35px rgba(28, 62, 59, 0.32))',
              }}
            />
          </div>
        </div>

        {/* Animated Mouse Cursor */}
        {localFrame < clickFrame + 26 && (
          <div
            style={{
              position: 'absolute',
              left: cursorX,
              top: cursorY,
              width: 28,
              height: 28,
              zIndex: 45,
              pointerEvents: 'none',
              transform: `scale(${cursorScale})`,
              filter: 'drop-shadow(0 4px 10px rgba(0, 0, 0, 0.35))',
            }}
          >
            <svg width="28" height="28" viewBox="0 0 24 24" fill={Pesto.textPrimary} stroke="#ffffff" strokeWidth="2">
              <path d="M3 3l7 18 3-7 7-3L3 3z" />
            </svg>
          </div>
        )}
      </div>
    </VrSpatialContainer>
  );
};

/**
 * =========================================================================
 * SCENE 4: AI AGENT CONTROL VIA MCP (Frames 790 - 985)
 * Transition: Enter from TOP -> Exit to RIGHT
 * Gap: 985 - 1000 (15 frames clean virtual gap)
 * Pure: Title + Subtitle (Doubled font size) + Cockpit Card (No eyebrows, no bottom asides)
 * =========================================================================
 */
export const ModernScene4Mcp: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();

  const localFrame = frame - 790;

  // Staggered springs (slightly slower, smoother)
  const staggerTitle = spring({frame: localFrame - 6, fps, config: {damping: 15, stiffness: 70, mass: 1.15}});
  const staggerSubtitle = spring({frame: localFrame - 18, fps, config: {damping: 15, stiffness: 75}});
  const staggerCard = spring({frame: localFrame - 30, fps, config: {damping: 14, stiffness: 65, mass: 1.2}});

  // Staggered inner card sequence
  const promptSpring = spring({frame: localFrame - 36, fps, config: {damping: 15, stiffness: 95}});
  const toolCallSpring = spring({frame: localFrame - 68, fps, config: {damping: 15, stiffness: 95}});
  const successSpring = spring({frame: localFrame - 104, fps, config: {damping: 15, stiffness: 95}});

  const pulseDot = Math.sin(localFrame / 8) > 0;

  return (
    <VrSpatialContainer
      enterStart={790}
      enterDuration={34}
      exitStart={960}
      exitDuration={25}
      enterDirection="from-top"
      exitDirection="to-right"
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        {/* Header: Title + Subtitle (Doubled font size, wraps cleanly) */}
        <div style={{textAlign: 'center', marginBottom: 24}}>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 58,
              fontWeight: 600,
              color: Pesto.textPrimary,
              letterSpacing: -1.4,
              lineHeight: 1.08,
              opacity: staggerTitle,
              transform: `translateY(${interpolate(staggerTitle, [0, 1], [20, 0])}px)`,
            }}
          >
            CONTROL WITH YOUR FAVORITE AI AGENT
          </div>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 42,
              color: Pesto.textSecondary,
              fontWeight: 450,
              marginTop: 12,
              maxWidth: 1150,
              lineHeight: 1.3,
              opacity: staggerSubtitle,
              transform: `translateY(${interpolate(staggerSubtitle, [0, 1], [15, 0])}px)`,
            }}
          >
            Native Model Context Protocol (MCP) server — Claude, Cursor, Windsurf, &amp; Antigravity.
          </div>
        </div>

        {/* 2.5D Agent Control Cockpit Card (No bottom aside) */}
        <div
          style={{
            position: 'relative',
            width: 1000,
            height: 480,
            borderRadius: 16,
            border: `1.5px solid ${Pesto.borderMedium}`,
            background: 'rgba(255, 255, 255, 0.75)',
            backdropFilter: 'blur(20px)',
            boxShadow: '0 25px 60px rgba(28, 62, 59, 0.18)',
            overflow: 'hidden',
            display: 'flex',
            flexDirection: 'column',
            opacity: staggerCard,
            transform: `perspective(1200px) rotateY(-2deg) rotateX(1.5deg) scale(${interpolate(staggerCard, [0, 1], [0.92, 1])})`,
          }}
        >
          {/* Window Chrome Header */}
          <div
            style={{
              height: 44,
              flexShrink: 0,
              borderBottom: `1px solid ${Pesto.borderSubtle}`,
              background: 'rgba(255, 255, 255, 0.85)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              padding: '0 18px',
            }}
          >
            <div style={{display: 'flex', alignItems: 'center', gap: 8}}>
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotClose}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMin}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMax}} />
              <span
                style={{
                  marginLeft: 12,
                  fontFamily: monoFont,
                  fontSize: 12,
                  fontWeight: 600,
                  color: Pesto.textSecondary,
                }}
              >
                tailorbird-mcp-server • http://127.0.0.1:42427/mcp
              </span>
            </div>

            {/* Frameless status markers */}
            <div style={{display: 'flex', alignItems: 'center', gap: 16}}>
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 7,
                  fontFamily: monoFont,
                  fontSize: 12,
                  fontWeight: 600,
                  color: '#065f46',
                }}
              >
                <span
                  style={{
                    width: 8,
                    height: 8,
                    borderRadius: 999,
                    background: Pesto.accentEmerald,
                    boxShadow: pulseDot ? '0 0 8px #10b981' : 'none',
                  }}
                />
                <span>MCP Ready :42427</span>
              </div>

              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 6,
                  fontFamily: monoFont,
                  fontSize: 12,
                  fontWeight: 600,
                  color: '#4338ca',
                }}
              >
                <span>↩</span>
                <span>Undo Stack (1)</span>
              </div>
            </div>
          </div>

          {/* Card Body */}
          <div
            style={{
              flex: 1,
              padding: '28px 32px',
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'center',
              gap: 16,
              background: 'transparent',
            }}
          >
            {/* Step 1: User Request to Agent */}
            <div
              style={{
                display: 'flex',
                alignItems: 'flex-start',
                gap: 12,
                padding: '14px 20px',
                borderRadius: 10,
                background: 'rgba(255, 255, 255, 0.9)',
                opacity: promptSpring,
                transform: `translateY(${interpolate(promptSpring, [0, 1], [15, 0])}px)`,
              }}
            >
              <div style={{fontSize: 20}}>🤖</div>
              <div style={{flex: 1}}>
                <div style={{fontFamily: monoFont, fontSize: 11, fontWeight: 700, color: Pesto.textDim, textTransform: 'uppercase'}}>
                  Agent Instruction (Cursor / Claude / Windsurf)
                </div>
                <div style={{fontFamily: outfitFont, fontSize: 16, fontWeight: 600, color: Pesto.textPrimary, marginTop: 2}}>
                  "Research open Flight Systems roles at Falcon Dynamics and sync the position to my Tailorbird prospects pipeline."
                </div>
              </div>
            </div>

            {/* Step 2: Agent MCP Tool Call */}
            <div
              style={{
                display: 'flex',
                alignItems: 'flex-start',
                gap: 12,
                padding: '14px 20px',
                borderRadius: 10,
                background: '#0f172a',
                boxShadow: '0 8px 20px rgba(15, 23, 42, 0.25)',
                opacity: toolCallSpring,
                transform: `translateY(${interpolate(toolCallSpring, [0, 1], [15, 0])}px)`,
              }}
            >
              <div style={{fontSize: 18}}>⚡</div>
              <div style={{flex: 1, overflow: 'hidden'}}>
                <div style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 4}}>
                  <span style={{fontFamily: monoFont, fontSize: 11, fontWeight: 700, color: '#38bdf8'}}>
                    MCP TOOL CALL: add_prospect
                  </span>
                  <span style={{fontFamily: monoFont, fontSize: 11, color: '#94a3b8'}}>JSON-RPC 2.0</span>
                </div>
                <pre
                  style={{
                    fontFamily: monoFont,
                    fontSize: 13,
                    color: '#e2e8f0',
                    lineHeight: 1.45,
                    margin: 0,
                  }}
                >
                  {`tailorbird.add_prospect({
  "company": "Falcon Dynamics",
  "jobTitle": "High Velocity Flight Engineer",
  "status": "Screening"
})`}
                </pre>
              </div>
            </div>

            {/* Step 3: Success & Safety Undo Notification */}
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                padding: '14px 20px',
                borderRadius: 10,
                background: 'rgba(16, 185, 129, 0.12)',
                opacity: successSpring,
                transform: `translateY(${interpolate(successSpring, [0, 1], [15, 0])}px)`,
              }}
            >
              <div style={{display: 'flex', alignItems: 'center', gap: 10}}>
                <span style={{fontSize: 18, color: '#047857', fontWeight: 'bold'}}>✓</span>
                <div>
                  <span style={{fontFamily: outfitFont, fontSize: 16, fontWeight: 700, color: '#065f46'}}>
                    Added "Falcon Dynamics — High Velocity Flight Engineer"
                  </span>
                  <div style={{fontFamily: monoFont, fontSize: 11, color: '#047857'}}>
                    Snapshot #1 recorded • Reversible with 1-click in the Tailorbird status bar
                  </div>
                </div>
              </div>
              <div
                style={{
                  fontFamily: outfitFont,
                  fontSize: 13,
                  fontWeight: 700,
                  padding: '4px 10px',
                  borderRadius: 6,
                  background: Pesto.accentEmerald,
                  color: '#ffffff',
                }}
              >
                100% Local &amp; Safe
              </div>
            </div>
          </div>
        </div>
      </div>
    </VrSpatialContainer>
  );
};

/**
 * =========================================================================
 * SCENE 5: PROSPECTS PIPELINE & PRIVACY (Frames 1000 - 1195)
 * Transition: Enter from LEFT -> Exit to DOWN
 * Gap: 1195 - 1210 (15 frames clean virtual gap)
 * Pure: Title + Subtitle (Doubled font size) + Screenshot (No eyebrows, no bottom asides)
 * =========================================================================
 */
export const ModernScene5Pipeline: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();

  const localFrame = frame - 1000;

  // Staggered springs (slightly slower, smoother)
  const staggerTitle = spring({frame: localFrame - 6, fps, config: {damping: 15, stiffness: 70, mass: 1.15}});
  const staggerSubtitle = spring({frame: localFrame - 18, fps, config: {damping: 15, stiffness: 75}});
  const staggerWindow = spring({frame: localFrame - 30, fps, config: {damping: 14, stiffness: 65, mass: 1.2}});
  const staggerMascot = spring({frame: localFrame - 46, fps, config: {damping: 15, stiffness: 85}});

  const driftY = Math.sin(localFrame / 22) * 4;

  return (
    <VrSpatialContainer
      enterStart={1000}
      enterDuration={34}
      exitStart={1170}
      exitDuration={25}
      enterDirection="from-left"
      exitDirection="to-down"
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        {/* Title Header: Title + Subtitle (Doubled font size, wraps cleanly) */}
        <div style={{textAlign: 'center', marginBottom: 24}}>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 60,
              fontWeight: 600,
              color: Pesto.textPrimary,
              letterSpacing: -1.4,
              lineHeight: 1.08,
              opacity: staggerTitle,
              transform: `translateY(${interpolate(staggerTitle, [0, 1], [20, 0])}px)`,
            }}
          >
            PROSPECTS PIPELINE
          </div>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 42,
              color: Pesto.textSecondary,
              fontWeight: 450,
              marginTop: 12,
              maxWidth: 1100,
              lineHeight: 1.3,
              opacity: staggerSubtitle,
              transform: `translateY(${interpolate(staggerSubtitle, [0, 1], [15, 0])}px)`,
            }}
          >
            Track every role from bookmark to offer. 100% offline, zero cloud accounts.
          </div>
        </div>

        {/* 2.5D Window Stage */}
        <div
          style={{
            position: 'relative',
            width: 1000,
            height: 480,
            opacity: staggerWindow,
            transform: `perspective(1200px) rotateY(-2.5deg) rotateX(1.5deg) translateY(${driftY}px) scale(${interpolate(staggerWindow, [0, 1], [0.92, 1])})`,
          }}
        >
          {/* macOS Window Chrome & Screenshot (Clipped) */}
          <div
            style={{
              position: 'relative',
              width: '100%',
              height: '100%',
              borderRadius: 16,
              border: `1.5px solid ${Pesto.borderMedium}`,
              background: 'rgba(255, 255, 255, 0.55)',
              backdropFilter: 'blur(20px)',
              boxShadow: Pesto.shadowHeavy,
              overflow: 'hidden',
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            {/* macOS Window Chrome */}
            <div
              style={{
                height: 38,
                flexShrink: 0,
                borderBottom: `1px solid ${Pesto.borderSubtle}`,
                background: 'rgba(255, 255, 255, 0.8)',
                display: 'flex',
                alignItems: 'center',
                padding: '0 18px',
                gap: 8,
              }}
            >
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotClose}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMin}} />
              <div style={{width: 10, height: 10, borderRadius: 999, background: Pesto.dotMax}} />
              <div
                style={{
                  marginLeft: 14,
                  fontFamily: monoFont,
                  fontSize: 12,
                  fontWeight: 600,
                  color: Pesto.textMuted,
                }}
              >
                Tailorbird — Prospects Pipeline &amp; Tracking
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

          {/* Mascot Head Tilt Peeking (Twice size: 280x280) on Top Right */}
          <div
            style={{
              position: 'absolute',
              top: -85,
              right: -35,
              width: 280,
              height: 280,
              zIndex: 35,
              pointerEvents: 'none',
              opacity: staggerMascot,
              transform: `translateY(${Math.sin(localFrame / 16) * 6}px)`,
            }}
          >
            <Img
              src={staticFile('images/bird-head-tilt-no-bg.png')}
              style={{
                width: '100%',
                height: '100%',
                objectFit: 'contain',
                filter: 'drop-shadow(0 20px 35px rgba(28, 62, 59, 0.32))',
              }}
            />
          </div>
        </div>
      </div>
    </VrSpatialContainer>
  );
};

/**
 * =========================================================================
 * SCENE 6: FINALE & DOWNLOADS (Frames 1210 - 1440)
 * Transition: Enter from DEPTH -> Final Hold
 * Pure: Title + Subtitle (Doubled font size) + Singing Mascot + Minimal Buttons
 * =========================================================================
 */
export const ModernScene6Finale: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();

  const localFrame = frame - 1210;

  // Staggered springs (slightly slower, smoother)
  const staggerTitle = spring({frame: localFrame - 6, fps, config: {damping: 15, stiffness: 70, mass: 1.15}});
  const staggerSubtitle = spring({frame: localFrame - 18, fps, config: {damping: 15, stiffness: 75}});
  const staggerMascot = spring({frame: localFrame - 30, fps, config: {damping: 14, stiffness: 65, mass: 1.2}});
  const staggerButtons = spring({frame: localFrame - 48, fps, config: {damping: 15, stiffness: 85}});

  return (
    <VrSpatialContainer
      enterStart={1210}
      enterDuration={34}
      exitStart={1440}
      exitDuration={10}
      enterDirection="from-depth"
      exitDirection="fade"
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        {/* Title Block: Title + Subtitle (Doubled font size, wraps cleanly) */}
        <div style={{textAlign: 'center', marginBottom: 24}}>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 68,
              fontWeight: 600,
              color: Pesto.textPrimary,
              letterSpacing: -1.6,
              lineHeight: 1.05,
              opacity: staggerTitle,
              transform: `translateY(${interpolate(staggerTitle, [0, 1], [20, 0])}px)`,
            }}
          >
            FLY THROUGH YOUR APPLICATIONS
          </div>
          <div
            style={{
              fontFamily: outfitFont,
              fontSize: 46,
              color: Pesto.textSecondary,
              fontWeight: 450,
              marginTop: 14,
              maxWidth: 1050,
              lineHeight: 1.3,
              opacity: staggerSubtitle,
              transform: `translateY(${interpolate(staggerSubtitle, [0, 1], [15, 0])}px)`,
            }}
          >
            Free &amp; open source desktop browser under the MIT License.
          </div>
        </div>

        {/* Mascot Singing Centerpiece with Musical Notes */}
        <div
          style={{
            position: 'relative',
            width: 250,
            height: 250,
            marginBottom: 32,
            opacity: staggerMascot,
            transform: `translateY(${Math.sin(localFrame / 12) * 6}px) scale(${interpolate(staggerMascot, [0, 1], [0.75, 1])})`,
          }}
        >
          <Img
            src={staticFile('images/bird-singing-no-bg.png')}
            style={{
              width: '100%',
              height: '100%',
              objectFit: 'contain',
              filter: 'drop-shadow(0 22px 38px rgba(28, 62, 59, 0.24))',
            }}
          />

          {/* Floating Musical Notes */}
          {[0, 1, 2].map((note, i) => {
            const noteY = (localFrame * 2.2 + i * 40) % 105;
            const noteX = Math.sin((localFrame + i * 22) / 12) * 28;
            const noteOpacity = Math.max(0, 1 - noteY / 100);

            return (
              <div
                key={note}
                style={{
                  position: 'absolute',
                  top: 40 - noteY,
                  right: 20 + noteX,
                  fontSize: 28,
                  color: Pesto.textPrimary,
                  opacity: noteOpacity,
                }}
              >
                🎵
              </div>
            );
          })}
        </div>

        {/* Minimal Platform Action Buttons per Rule #4 (Clean, minimal, no pill strokes) */}
        <div
          style={{
            display: 'flex',
            gap: 22,
            justifyContent: 'center',
            alignItems: 'center',
            opacity: staggerButtons,
            transform: `translateY(${interpolate(staggerButtons, [0, 1], [15, 0])}px)`,
          }}
        >
          {/* Windows Button */}
          <div
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: 10,
              padding: '13px 30px',
              borderRadius: 8,
              background: Pesto.textPrimary,
              color: '#ffffff',
              fontFamily: outfitFont,
              fontSize: 18,
              fontWeight: 600,
              boxShadow: '0 8px 20px rgba(28, 62, 59, 0.22)',
            }}
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-12.949-1.801" />
            </svg>
            <span>Windows</span>
            <span style={{opacity: 0.7}}>↓</span>
          </div>

          {/* macOS Button */}
          <div
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: 10,
              padding: '13px 30px',
              borderRadius: 8,
              background: 'rgba(255, 255, 255, 0.85)',
              color: Pesto.textPrimary,
              fontFamily: outfitFont,
              fontSize: 18,
              fontWeight: 600,
              boxShadow: Pesto.shadowLight,
            }}
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M15.97 4.07c.65-.79 1.09-1.89.97-2.99-1 .04-2.2 1-2.91 1.83-.59.69-1.12 1.8-1 2.88 1.13.09 2.27-.88 2.94-1.72z" />
            </svg>
            <span>macOS</span>
            <span style={{opacity: 0.7}}>↓</span>
          </div>

          {/* Linux Button */}
          <div
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: 10,
              padding: '12px 28px',
              borderRadius: 8,
              background: 'rgba(255, 255, 255, 0.85)',
              color: Pesto.textPrimary,
              fontFamily: outfitFont,
              fontSize: 18,
              fontWeight: 600,
              boxShadow: Pesto.shadowLight,
            }}
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12.001 2c-2.438 0-4.417 2.05-4.417 4.57 0 .809.213 1.57.585 2.226C7.306 9.336 6.5 10.59 6.5 12.06c0 1.066.425 2.032 1.11 2.748-.06.38-.095.77-.095 1.171 0 3.328 2.012 6.021 4.486 6.021 2.475 0 4.487-2.693 4.487-6.021 0-.4-.035-.79-.095-1.171.684-.716 1.11-1.682 1.11-2.748 0-1.47-.806-2.724-1.669-3.264.372-.656.585-1.417.585-2.226C16.418 4.05 14.439 2 12.001 2z" />
            </svg>
            <span>Linux</span>
            <span style={{opacity: 0.7}}>↓</span>
          </div>
        </div>
      </div>
    </VrSpatialContainer>
  );
};
