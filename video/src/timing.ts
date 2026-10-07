export const FPS = 30;
export const TOTAL_FRAMES = 900; // 30 seconds

/**
 * 360-Degree Panoramic Forest Rotation:
 *
 * Total cycle = 900 frames across 5 segments.
 * Each segment = 180 frames:
 *   - Clearing Hold: 0 to 95 frames (~3.16s)
 *       * Continuous gentle panoramic drift at V_DRIFT (0.16 deg/frame)
 *       * Content reveals cleanly and holds for readability
 *       * Fades out completely by frame 95
 *   - Transition Phase: 95 to 180 frames (~2.83s)
 *       * Continuous acceleration and deceleration with C^1 velocity matching (ZERO jerk, perfectly smooth ease-in / ease-out)
 *       * Foreground shadow tree sweeps across starting right as slide contents fade out (frame 88)
 *         and finishes leaving right as new contents arrive (frame 188 / 8)
 *
 * Seamless 360° Loop:
 * Over the 900 frames, total pan angle rotates exactly 360°, matching frame 0 and frame 900 identically.
 */

export interface ClearingData {
  id: string;
  clearingIndex: number;
  angle: number;
  title: string;
  subtitle: string;
}

export const CLEARINGS: ClearingData[] = [
  {
    id: 'intro',
    clearingIndex: 0,
    angle: 0,
    title: 'TAILORBIRD',
    subtitle: 'The browser built for the job search.',
  },
  {
    id: 'problem',
    clearingIndex: 1,
    angle: 72,
    title: 'TIRED OF TAB CHAOS?',
    subtitle: '37 open tabs. 4 spreadsheets. Zero sanity.',
  },
  {
    id: 'workspace',
    clearingIndex: 2,
    angle: 144,
    title: 'DUAL-PANE WORKSPACE',
    subtitle: 'Candidate profile on the left. Live applications on the right.',
  },
  {
    id: 'autofill',
    clearingIndex: 3,
    angle: 216,
    title: '1-CLICK AUTOFILL',
    subtitle: 'Stop typing your name for the 400th time.',
  },
  {
    id: 'free',
    clearingIndex: 4,
    angle: 288,
    title: '100% FREE & PRIVATE',
    subtitle: 'No subscriptions. No tracking. Your data stays on your machine.',
  },
];

export function getPanAngle(frame: number): {
  panAngle: number;
  currentClearingIdx: number;
  isPaused: boolean;
  slideProgress: number;
} {
  const normFrame = ((frame % TOTAL_FRAMES) + TOTAL_FRAMES) % TOTAL_FRAMES;

  const segmentLength = 180; // 900 / 5
  const segmentIdx = Math.floor(normFrame / segmentLength);
  const frameInSegment = normFrame % segmentLength;

  const holdDuration = 95;
  const turnDuration = segmentLength - holdDuration; // 85 frames

  const baseAngle = segmentIdx * 72;
  const V_DRIFT = 0.16; // constant base drift rate in deg/frame
  const EXTRA_DEG = 72.0 - segmentLength * V_DRIFT; // 43.2 degrees

  if (frameInSegment < holdDuration) {
    // Gentle continuous drift: perfectly constant velocity
    const currentAngle = baseAngle + frameInSegment * V_DRIFT;

    return {
      panAngle: currentAngle,
      currentClearingIdx: segmentIdx,
      isPaused: true,
      slideProgress: frameInSegment,
    };
  } else {
    // Perfectly smooth transition with zero jerk and continuous velocity at start & end
    const tau = (frameInSegment - holdDuration) / turnDuration; // 0 to 1
    // Smooth ease: tau - sin(2*pi*tau) / (2*pi)
    const smoothEase = tau - Math.sin(2 * Math.PI * tau) / (2 * Math.PI);
    const currentAngle = baseAngle + frameInSegment * V_DRIFT + EXTRA_DEG * smoothEase;

    return {
      panAngle: currentAngle,
      currentClearingIdx: segmentIdx,
      isPaused: false,
      slideProgress: -1,
    };
  }
}
