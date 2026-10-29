import React from 'react';
import {interpolate, spring, useCurrentFrame, useVideoConfig} from 'remotion';

export type EnterDirection =
  | 'from-center'
  | 'from-right'
  | 'from-bottom'
  | 'from-top'
  | 'from-left'
  | 'from-depth';

export type ExitDirection =
  | 'to-left'
  | 'to-up'
  | 'to-right'
  | 'to-down'
  | 'to-depth'
  | 'fade';

export interface VrSpatialContainerProps {
  children: React.ReactNode;
  enterStart: number;
  enterDuration?: number; // default: 28 frames
  exitStart: number;
  exitDuration?: number;  // default: 24 frames
  enterDirection?: EnterDirection;
  exitDirection?: ExitDirection;
}

/**
 * VrSpatialContainer:
 * Handles directional 3D transitions with ease-in bounce settles,
 * realistic motion blur, and guaranteed clear gap between slide contents.
 */
export const VrSpatialContainer: React.FC<VrSpatialContainerProps> = ({
  children,
  enterStart,
  enterDuration = 28,
  exitStart,
  exitDuration = 24,
  enterDirection = 'from-right',
  exitDirection = 'to-left',
}) => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();

  const exitEnd = exitStart + exitDuration;

  // Strict culling: completely unmount before enterStart and after exitEnd
  if (frame < enterStart || frame > exitEnd) {
    return null;
  }

  const isExiting = frame >= exitStart;

  // 1. Enter Spring: organic ease-in with bounce overshoot (slower, smoother pacing)
  const enterSpring = spring({
    frame: frame - enterStart,
    fps,
    config: {
      damping: 14,    // Smooth, controlled overshoot bounce
      stiffness: 55,  // Slower, more cinematic gliding entrance
      mass: 1.2,
    },
  });

  const enterFrac = Math.min(1, Math.max(0, (frame - enterStart) / enterDuration));
  const enterBlur = enterFrac < 1 ? Math.sin(enterFrac * Math.PI) * 7.5 : 0;

  let enterTx = 0;
  let enterTy = 0;
  let enterTz = 0;
  let enterRx = 0;
  let enterRy = 0;
  let enterScale = 1;
  const enterOpacity = interpolate(enterSpring, [0, 0.35, 1], [0, 1, 1], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });

  switch (enterDirection) {
    case 'from-right':
      enterTx = interpolate(enterSpring, [0, 1], [1450, 0]);
      enterRy = interpolate(enterSpring, [0, 1], [35, 0]);
      enterTz = interpolate(enterSpring, [0, 1], [-300, 0]);
      enterScale = interpolate(enterSpring, [0, 1], [0.85, 1]);
      break;
    case 'from-bottom':
      enterTy = interpolate(enterSpring, [0, 1], [950, 0]);
      enterRx = interpolate(enterSpring, [0, 1], [-25, 0]);
      enterTz = interpolate(enterSpring, [0, 1], [-250, 0]);
      enterScale = interpolate(enterSpring, [0, 1], [0.88, 1]);
      break;
    case 'from-top':
      enterTy = interpolate(enterSpring, [0, 1], [-950, 0]);
      enterRx = interpolate(enterSpring, [0, 1], [25, 0]);
      enterTz = interpolate(enterSpring, [0, 1], [-250, 0]);
      enterScale = interpolate(enterSpring, [0, 1], [0.88, 1]);
      break;
    case 'from-left':
      enterTx = interpolate(enterSpring, [0, 1], [-1450, 0]);
      enterRy = interpolate(enterSpring, [0, 1], [-35, 0]);
      enterTz = interpolate(enterSpring, [0, 1], [-300, 0]);
      enterScale = interpolate(enterSpring, [0, 1], [0.85, 1]);
      break;
    case 'from-depth':
      enterTz = interpolate(enterSpring, [0, 1], [-600, 0]);
      enterScale = interpolate(enterSpring, [0, 1], [0.65, 1]);
      break;
    case 'from-center':
    default:
      enterScale = interpolate(enterSpring, [0, 1], [0.85, 1]);
      break;
  }

  // 2. Exit Spring: accelerates cleanly away
  const exitSpring = isExiting
    ? spring({
        frame: frame - exitStart,
        fps,
        config: {
          damping: 14,
          stiffness: 85,
          mass: 0.95,
        },
      })
    : 0;

  const exitFrac = isExiting
    ? Math.min(1, Math.max(0, (frame - exitStart) / exitDuration))
    : 0;
  const exitBlur = isExiting && exitFrac < 1 ? Math.sin(exitFrac * Math.PI) * 8.5 : 0;

  let exitTx = 0;
  let exitTy = 0;
  let exitTz = 0;
  let exitRx = 0;
  let exitRy = 0;
  let exitScale = 1;
  const exitOpacity = isExiting
    ? interpolate(exitSpring, [0, 0.65, 1], [1, 0.35, 0], {
        extrapolateLeft: 'clamp',
        extrapolateRight: 'clamp',
      })
    : 1;

  switch (exitDirection) {
    case 'to-left':
      exitTx = interpolate(exitSpring, [0, 1], [0, -1450]);
      exitRy = interpolate(exitSpring, [0, 1], [0, -35]);
      exitTz = interpolate(exitSpring, [0, 1], [0, -300]);
      exitScale = interpolate(exitSpring, [0, 1], [1, 0.85]);
      break;
    case 'to-up':
      exitTy = interpolate(exitSpring, [0, 1], [0, -950]);
      exitRx = interpolate(exitSpring, [0, 1], [0, 25]);
      exitTz = interpolate(exitSpring, [0, 1], [0, -250]);
      exitScale = interpolate(exitSpring, [0, 1], [1, 0.88]);
      break;
    case 'to-right':
      exitTx = interpolate(exitSpring, [0, 1], [0, 1450]);
      exitRy = interpolate(exitSpring, [0, 1], [0, 35]);
      exitTz = interpolate(exitSpring, [0, 1], [0, -300]);
      exitScale = interpolate(exitSpring, [0, 1], [1, 0.85]);
      break;
    case 'to-down':
      exitTy = interpolate(exitSpring, [0, 1], [0, 950]);
      exitRx = interpolate(exitSpring, [0, 1], [0, -25]);
      exitTz = interpolate(exitSpring, [0, 1], [0, -250]);
      exitScale = interpolate(exitSpring, [0, 1], [1, 0.88]);
      break;
    case 'to-depth':
      exitTz = interpolate(exitSpring, [0, 1], [0, -600]);
      exitScale = interpolate(exitSpring, [0, 1], [1, 0.65]);
      break;
    case 'fade':
    default:
      exitScale = interpolate(exitSpring, [0, 1], [1, 0.9]);
      break;
  }

  // Combined coordinates
  const tx = isExiting ? exitTx : enterTx;
  const ty = isExiting ? exitTy : enterTy;
  const tz = isExiting ? exitTz : enterTz;
  const rx = isExiting ? exitRx : enterRx;
  const ry = isExiting ? exitRy : enterRy;
  const scale = isExiting ? exitScale : enterScale;
  const opacity = Math.max(0, Math.min(1, isExiting ? exitOpacity : enterOpacity));
  const blur = Math.max(enterBlur, exitBlur);

  // Subtle ambient float when resting in center
  const ambientFloatY = Math.sin((frame - enterStart) / 18) * 3;

  return (
    <div
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        perspective: 1400,
        perspectiveOrigin: '50% 50%',
        transformStyle: 'preserve-3d',
        opacity,
        filter: blur > 0.4 ? `blur(${blur.toFixed(1)}px)` : undefined,
        transform: `translate3d(${tx}px, ${ty + ambientFloatY}px, ${tz}px) rotateX(${rx}deg) rotateY(${ry}deg) scale(${scale})`,
        pointerEvents: isExiting ? 'none' : 'auto',
      }}
    >
      {children}
    </div>
  );
};
