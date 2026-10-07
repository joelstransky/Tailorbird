import React from 'react';
import {interpolate, spring, useCurrentFrame, useVideoConfig} from 'remotion';
import {Dusk, display, body} from './theme';

/**
 * Broadcast-grade typography title
 * Punchy typography slam reveal with clean drop shadows and no web containers.
 */
export const BroadcastTitle: React.FC<{
  title: string;
  subtitle: string;
  slideProgress: number; // 0 to 110
}> = ({title, subtitle, slideProgress}) => {
  const {fps} = useVideoConfig();

  // Entrance spring
  const s = spring({
    frame: slideProgress,
    fps,
    config: {damping: 12, stiffness: 150},
  });

  const titleScale = interpolate(s, [0, 1], [0.9, 1.0]);
  const titleY = interpolate(s, [0, 1], [25, 0]);

  // Subtitle reveals right after
  const subSpring = spring({
    frame: slideProgress - 4,
    fps,
    config: {damping: 14, stiffness: 140},
  });

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        textAlign: 'center',
        marginBottom: 34,
      }}
    >
      <div
        style={{
          fontFamily: display,
          fontWeight: 900,
          fontSize: 76,
          lineHeight: 1.05,
          letterSpacing: -2,
          color: Dusk.textLight,
          opacity: s,
          transform: `translateY(${titleY}px) scale(${titleScale})`,
          textShadow: `0 6px 40px rgba(0, 0, 0, 0.95), 0 0 50px ${Dusk.accentOrange}55`,
          textTransform: 'uppercase',
        }}
      >
        {title}
      </div>

      <div
        style={{
          fontFamily: body,
          fontWeight: 700,
          fontSize: 26,
          lineHeight: 1.3,
          letterSpacing: 0.5,
          color: Dusk.accentYellow,
          opacity: subSpring,
          transform: `translateY(${interpolate(subSpring, [0, 1], [15, 0])}px)`,
          textShadow: '0 4px 25px rgba(0, 0, 0, 0.95)',
          marginTop: 12,
          maxWidth: 950,
        }}
      >
        {subtitle}
      </div>
    </div>
  );
};

/**
 * Broadcast 2.5D Dual-Pane Workspace Showcase
 * Shows the candidate profile properties on the left and active job form on the right.
 */
export const BroadcastDualPane: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 6,
    fps,
    config: {damping: 13, stiffness: 130},
  });

  // Floating ambient drift
  const driftY = Math.sin(slideProgress / 18) * 6;
  const driftRotate = Math.cos(slideProgress / 22) * 0.8;

  const tiltStyle: React.CSSProperties = {
    transform: `perspective(1200px) rotateY(-6deg) rotateX(4deg) rotateZ(${driftRotate}deg) translateY(${driftY}px) scale(${interpolate(entrance, [0, 1], [0.85, 1])})`,
    opacity: entrance,
    transformStyle: 'preserve-3d',
    boxShadow: '0 50px 100px -20px rgba(0,0,0,0.85), 0 0 60px rgba(249, 115, 22, 0.25)',
    borderRadius: 18,
    border: '2px solid rgba(255, 255, 255, 0.15)',
    overflow: 'hidden',
    width: 1100,
    height: 520,
    background: '#160d29',
    display: 'flex',
    flexDirection: 'column',
  };

  return (
    <div style={tiltStyle}>
      {/* Broadcast Window Topbar */}
      <div
        style={{
          height: 44,
          background: '#0d061a',
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
          display: 'flex',
          alignItems: 'center',
          padding: '0 20px',
          gap: 10,
        }}
      >
        <div style={{width: 12, height: 12, borderRadius: 6, background: '#ef4444'}} />
        <div style={{width: 12, height: 12, borderRadius: 6, background: '#f59e0b'}} />
        <div style={{width: 12, height: 12, borderRadius: 6, background: '#10b981'}} />
        <div
          style={{
            marginLeft: 18,
            fontFamily: body,
            fontSize: 14,
            fontWeight: 700,
            color: '#a78bfa',
            letterSpacing: 1.5,
          }}
        >
          TAILORBIRD // DUAL-PANE COCKPIT
        </div>
      </div>

      {/* Split Panels */}
      <div style={{display: 'flex', flex: 1}}>
        {/* Left Pane: Candidate Profile */}
        <div
          style={{
            width: 380,
            background: '#1a1033',
            borderRight: '1.5px solid rgba(255, 255, 255, 0.12)',
            padding: 24,
            fontFamily: body,
          }}
        >
          <div
            style={{
              fontSize: 13,
              fontWeight: 800,
              letterSpacing: 2,
              color: '#facc15',
              marginBottom: 16,
            }}
          >
            CANDIDATE PROFILE
          </div>

          {[
            ['Full Name', 'Robin Featherstone'],
            ['Email', 'robin@maplegrove.dev'],
            ['Phone', '(555) 247-3637'], // (555) BIRD-NEST
            ['Target Role', 'Senior Nest Architect'],
            ['Location', 'Maple Grove Canopy / Remote'],
          ].map(([k, v], i) => (
            <div key={k} style={{marginBottom: 12}}>
              <div style={{fontSize: 12, color: '#9ca3af', fontWeight: 600, marginBottom: 3}}>{k}</div>
              <div
                style={{
                  background: 'rgba(255, 255, 255, 0.07)',
                  borderRadius: 6,
                  padding: '7px 12px',
                  fontSize: 14,
                  fontWeight: 600,
                  color: '#fff',
                  border: '1px solid rgba(255, 255, 255, 0.1)',
                }}
              >
                {v}
              </div>
            </div>
          ))}
        </div>

        {/* Right Pane: Live Web Application */}
        <div
          style={{
            flex: 1,
            background: '#231642',
            padding: 28,
            fontFamily: body,
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <div style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 20}}>
            <div>
              <div style={{fontSize: 18, fontWeight: 800, color: '#fff'}}>Greenhouse Application</div>
              <div style={{fontSize: 13, color: '#c4b5fd'}}>Wren Industries · Senior Nest Architect</div>
            </div>
            <div
              style={{
                background: '#f97316',
                color: '#fff',
                fontSize: 12,
                fontWeight: 800,
                padding: '6px 14px',
                borderRadius: 6,
                letterSpacing: 1,
              }}
            >
              PARALLEL VIEW
            </div>
          </div>

          <div
            style={{
              flex: 1,
              background: 'rgba(0, 0, 0, 0.3)',
              borderRadius: 10,
              border: '1px solid rgba(255, 255, 255, 0.08)',
              padding: 20,
              display: 'flex',
              flexDirection: 'column',
              gap: 14,
            }}
          >
            <div style={{height: 18, width: '40%', background: 'rgba(255, 255, 255, 0.15)', borderRadius: 4}} />
            <div style={{height: 38, background: 'rgba(255, 255, 255, 0.08)', borderRadius: 6}} />
            <div style={{height: 18, width: '30%', background: 'rgba(255, 255, 255, 0.15)', borderRadius: 4}} />
            <div style={{height: 38, background: 'rgba(255, 255, 255, 0.08)', borderRadius: 6}} />
          </div>
        </div>
      </div>
    </div>
  );
};

/**
 * Broadcast 1-Click Autofill Demonstration
 * Live cursor clicks Autofill button, fields fill with typing effect + green success glow.
 */
export const BroadcastAutofill: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 6,
    fps,
    config: {damping: 13, stiffness: 130},
  });

  const CLICK_FRAME = 35;
  const isClicked = slideProgress >= CLICK_FRAME;

  // Cursor trajectory towards Autofill button
  const cursorX = interpolate(slideProgress, [12, CLICK_FRAME], [850, 410], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });
  const cursorY = interpolate(slideProgress, [12, CLICK_FRAME], [380, 88], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });
  const isClicking = slideProgress >= CLICK_FRAME && slideProgress < CLICK_FRAME + 6;

  const FORM_FIELDS = [
    {label: 'Candidate Name', val: 'Robin Featherstone'},
    {label: 'Direct Email', val: 'robin@maplegrove.dev'},
    {label: 'Phone Number', val: '(555) 247-3637'},
    {label: 'Aviary Profile', val: 'aviary.network/in/robin-featherstone'},
  ];

  return (
    <div
      style={{
        position: 'relative',
        width: 1040,
        height: 500,
        borderRadius: 18,
        background: '#160d29',
        border: '2px solid rgba(255, 255, 255, 0.15)',
        boxShadow: '0 50px 100px -20px rgba(0,0,0,0.85), 0 0 60px rgba(234, 179, 8, 0.25)',
        transform: `perspective(1200px) rotateY(5deg) rotateX(3deg) scale(${interpolate(entrance, [0, 1], [0.85, 1])})`,
        opacity: entrance,
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
      }}
    >
      {/* Top Header */}
      <div
        style={{
          height: 64,
          background: '#0e061c',
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '0 28px',
        }}
      >
        <div style={{fontFamily: body, fontWeight: 800, fontSize: 16, color: '#fff', letterSpacing: 1}}>
          AUTOMATED FORM DISPATCH
        </div>

        {/* The Action Button */}
        <div
          style={{
            fontFamily: body,
            fontWeight: 800,
            fontSize: 14,
            letterSpacing: 1.5,
            padding: '10px 24px',
            borderRadius: 8,
            background: isClicked ? '#22c55e' : '#f97316',
            color: '#fff',
            transform: `scale(${isClicking ? 0.92 : 1})`,
            boxShadow: isClicked ? '0 0 35px rgba(34, 197, 94, 0.8)' : '0 0 25px rgba(249, 115, 22, 0.5)',
            transition: 'background 0.2s',
          }}
        >
          {isClicked ? '✓ AUTOFILLED' : '⚡ 1-CLICK AUTOFILL'}
        </div>
      </div>

      {/* Live Populating Form */}
      <div
        style={{
          flex: 1,
          padding: 34,
          display: 'grid',
          gridTemplateColumns: '1fr 1fr',
          gap: '24px 30px',
          fontFamily: body,
        }}
      >
        {FORM_FIELDS.map((f, i) => {
          const typeStart = CLICK_FRAME + 4 + i * 5;
          const charsTyped = Math.floor(
            interpolate(slideProgress, [typeStart, typeStart + 12], [0, f.val.length], {
              extrapolateLeft: 'clamp',
              extrapolateRight: 'clamp',
            })
          );
          const isDone = charsTyped >= f.val.length;

          return (
            <div key={f.label}>
              <div style={{fontSize: 13, fontWeight: 700, color: '#9ca3af', marginBottom: 6}}>
                {f.label}
              </div>
              <div
                style={{
                  height: 48,
                  borderRadius: 8,
                  background: 'rgba(255, 255, 255, 0.06)',
                  border: `2px solid ${isDone ? '#22c55e' : 'rgba(255, 255, 255, 0.15)'}`,
                  display: 'flex',
                  alignItems: 'center',
                  padding: '0 16px',
                  fontSize: 17,
                  fontWeight: 600,
                  color: isDone ? '#fff' : '#6b7280',
                  boxShadow: isDone ? '0 0 20px rgba(34, 197, 94, 0.35)' : 'none',
                }}
              >
                {charsTyped > 0 ? f.val.slice(0, charsTyped) : 'Empty'}
              </div>
            </div>
          );
        })}
      </div>

      {/* Animated Mouse Cursor */}
      <svg
        width={36}
        height={36}
        viewBox="0 0 24 24"
        style={{
          position: 'absolute',
          left: cursorX,
          top: cursorY,
          transform: `scale(${isClicking ? 0.82 : 1})`,
          filter: 'drop-shadow(0 4px 10px rgba(0,0,0,0.8))',
          pointerEvents: 'none',
          zIndex: 30,
        }}
      >
        <path d="M3 2l7 19 3-8 8-3z" fill="#ffffff" stroke="#000000" strokeWidth={1.5} />
      </svg>
    </div>
  );
};

/**
 * Broadcast Prospects Pipeline & Privacy Guarantee
 */
export const BroadcastProspects: React.FC<{slideProgress: number}> = ({slideProgress}) => {
  const {fps} = useVideoConfig();

  const entrance = spring({
    frame: slideProgress - 6,
    fps,
    config: {damping: 13, stiffness: 130},
  });

  const PROSPECTS = [
    {co: 'Wren Industries', role: 'Senior Nest Architect', status: 'Applied', color: '#38bdf8'},
    {co: 'Owl Analytics', role: 'Staff Night Vision Engineer', status: 'Interviewing', color: '#4ade80'},
    {co: 'Falcon Dynamics', role: 'High Velocity Lead', status: 'Offer', color: '#facc15'},
  ];

  return (
    <div
      style={{
        width: 1040,
        height: 480,
        borderRadius: 18,
        background: '#160d29',
        border: '2px solid rgba(255, 255, 255, 0.15)',
        boxShadow: '0 50px 100px -20px rgba(0,0,0,0.85), 0 0 60px rgba(56, 189, 248, 0.25)',
        transform: `perspective(1200px) rotateY(-4deg) rotateX(2deg) scale(${interpolate(entrance, [0, 1], [0.85, 1])})`,
        opacity: entrance,
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
        fontFamily: body,
      }}
    >
      <div
        style={{
          height: 60,
          background: '#0d061a',
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '0 28px',
        }}
      >
        <div style={{fontWeight: 800, fontSize: 16, color: '#fff', letterSpacing: 1}}>
          LOCAL PROSPECTS PIPELINE
        </div>
        <div style={{color: '#a78bfa', fontSize: 13, fontWeight: 700}}>
          LOCAL JSON ENCRYPTED ON YOUR DEVICE
        </div>
      </div>

      <div style={{padding: 28, flex: 1, display: 'flex', flexDirection: 'column', gap: 14}}>
        {PROSPECTS.map((p, i) => {
          const rowSpring = spring({
            frame: slideProgress - 12 - i * 6,
            fps,
            config: {damping: 14, stiffness: 140},
          });

          return (
            <div
              key={p.co}
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                padding: '18px 24px',
                borderRadius: 10,
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                opacity: rowSpring,
                transform: `translateX(${interpolate(rowSpring, [0, 1], [40, 0])}px)`,
              }}
            >
              <div>
                <div style={{fontSize: 20, fontWeight: 800, color: '#fff'}}>{p.co}</div>
                <div style={{fontSize: 14, color: '#9ca3af', fontWeight: 600}}>{p.role}</div>
              </div>

              <div
                style={{
                  padding: '8px 18px',
                  borderRadius: 9999,
                  background: `${p.color}22`,
                  border: `1.5px solid ${p.color}`,
                  color: p.color,
                  fontSize: 14,
                  fontWeight: 800,
                  letterSpacing: 1,
                }}
              >
                {p.status}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
