import {Composition} from 'remotion';
import {Launch} from './Launch';
import {FPS, TOTAL_FRAMES} from './timing';

export const Root: React.FC = () => (
  <Composition
    id="Launch"
    component={Launch}
    durationInFrames={TOTAL_FRAMES}
    fps={FPS}
    width={1920}
    height={1080}
  />
);
