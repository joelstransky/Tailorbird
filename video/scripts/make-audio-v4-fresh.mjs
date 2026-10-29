// make-audio-v4-fresh.mjs
// Built from scratch for Tailorbird 48-second Cinematic Showcase.
// Tempo: 110 BPM. Total duration: exactly 48.000 seconds (88 beats = 22 bars).
// Style: Uplifting Ambient Neo-Soul / Future Tech Beats.
// Palette: Lush Rhodes FM keys, deep saturated sub-bass, organic wood snap,
// crisp lo-fi kick, brushed hats, and glassy marimba arpeggio accents.

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const SAMPLE_RATE = 44100;
const DURATION_SECS = 48.0;
const TOTAL_SAMPLES = Math.floor(SAMPLE_RATE * DURATION_SECS);
const BPM = 110;
const BEAT_SECS = 60 / BPM;
const SAMPLE_PER_BEAT = SAMPLE_RATE * BEAT_SECS;

const left = new Float32Array(TOTAL_SAMPLES);
const right = new Float32Array(TOTAL_SAMPLES);

// Note frequencies (Hz) in Eb Major / Ab Major palette
const N = {
  // Bass
  Eb1: 38.89, F1: 43.65, G1: 49.00, Ab1: 51.91, Bb1: 58.27, C2: 65.41, D2: 73.42, Eb2: 77.78, F2: 87.31,
  // Mid Chords
  G3: 196.00, Ab3: 207.65, Bb3: 233.08, C4: 261.63, D4: 293.66, Eb4: 311.13, F4: 349.23, G4: 392.00, Ab4: 415.30, Bb4: 466.16,
  // High Accents
  C5: 523.25, D5: 587.33, Eb5: 622.25, F5: 698.46, G5: 783.99, Bb5: 932.33, C6: 1046.50
};

// 4-Bar Chord Loop: Abmaj9 -> Bb6/C -> Fm9 -> Ebmaj9
const CHORDS = [
  { root: N.Ab1, notes: [N.Ab3, N.C4, N.Eb4, N.G4, N.Bb4] }, // Abmaj9
  { root: N.C2,  notes: [N.Bb3, N.D4, N.F4, N.Bb4] },         // Bb/C
  { root: N.F1,  notes: [N.Ab3, N.C4, N.Eb4, N.G4] },         // Fm9
  { root: N.Eb1, notes: [N.G3, N.Bb3, N.D4, N.F4] },          // Ebmaj9
];

function mix(startSample, durSamples, generator, volL = 0.5, volR = 0.5) {
  const end = Math.min(TOTAL_SAMPLES, startSample + durSamples);
  for (let i = startSample; i < end; i++) {
    const t = (i - startSample) / SAMPLE_RATE;
    const s = generator(t, i - startSample);
    left[i] += s * volL;
    right[i] += s * volR;
  }
}

// 1. DRUMS
function renderKick(startSample) {
  const dur = Math.floor(SAMPLE_RATE * 0.32);
  mix(startSample, dur, (t) => {
    const pitch = 52 + 90 * Math.exp(-t * 32);
    const phase = 2 * Math.PI * (52 * t - (90 / 32) * Math.exp(-t * 32));
    const env = Math.exp(-t * 10);
    const click = Math.exp(-t * 140) * (Math.random() * 2 - 1) * 0.15;
    return (Math.sin(phase) * env + click) * 0.85;
  }, 0.65, 0.65);
}

function renderWoodSnap(startSample) {
  const dur = Math.floor(SAMPLE_RATE * 0.2);
  mix(startSample, dur, (t) => {
    const tone = (Math.sin(2 * Math.PI * 460 * t) * 0.6 + Math.sin(2 * Math.PI * 920 * t) * 0.25) * Math.exp(-t * 26);
    const snap = (Math.random() * 2 - 1) * Math.exp(-t * 42) * 0.45;
    return (tone + snap) * 0.6;
  }, 0.48, 0.52);
}

function renderHiHat(startSample, accented = false) {
  const dur = Math.floor(SAMPLE_RATE * (accented ? 0.07 : 0.04));
  const amp = accented ? 0.22 : 0.13;
  mix(startSample, dur, (t) => {
    const decay = accented ? 38 : 60;
    const noise = (Math.random() * 2 - 1) * Math.exp(-t * decay);
    const metallic = Math.sin(2 * Math.PI * 7200 * t) * 0.12 * Math.exp(-t * decay);
    return (noise + metallic) * amp;
  }, 0.44, 0.56);
}

function renderShaker(startSample) {
  const dur = Math.floor(SAMPLE_RATE * 0.045);
  mix(startSample, dur, (t) => {
    const env = Math.sin(Math.PI * (t / (dur / SAMPLE_RATE)));
    return (Math.random() * 2 - 1) * env * 0.1;
  }, 0.58, 0.42);
}

// 2. RHODES FM KEYS
function renderRhodesChord(startSample, durBeats, notes, velocity = 0.45) {
  const durSamples = Math.floor(SAMPLE_PER_BEAT * durBeats);
  notes.forEach((freq, idx) => {
    const panOffset = (idx - 1.5) * 0.14;
    const volL = (0.5 - panOffset) * velocity;
    const volR = (0.5 + panOffset) * velocity;

    mix(startSample, durSamples, (t) => {
      // FM synthesis: Modulator modulates Carrier
      const modIndex = 1.6 * Math.exp(-t * 3.5);
      const mod = Math.sin(2 * Math.PI * freq * t) * modIndex;
      const carrier = Math.sin(2 * Math.PI * freq * t + mod);
      // Bell overtone
      const bell = Math.sin(2 * Math.PI * freq * 4.2 * t) * 0.08 * Math.exp(-t * 12);
      // Soft attack & organic decay
      const attack = Math.min(1, t / 0.012);
      const decay = Math.exp(-t * (1.2 / durBeats));
      const tremolo = 1 + 0.04 * Math.sin(2 * Math.PI * 4 * t);
      return (carrier * 0.8 + bell) * attack * decay * tremolo * 0.35;
    }, volL, volR);
  });
}

// 3. MOOG SUB-BASS
function renderBass(startSample, durBeats, freq, velocity = 0.55) {
  const durSamples = Math.floor(SAMPLE_PER_BEAT * durBeats);
  mix(startSample, durSamples, (t) => {
    const sub = Math.sin(2 * Math.PI * freq * t);
    const subOct = Math.sin(2 * Math.PI * (freq / 2) * t) * 0.28;
    const harm2 = Math.sin(2 * Math.PI * freq * 2 * t) * 0.22;
    const attack = Math.min(1, t / 0.01);
    const decay = Math.exp(-t * 2.0);
    const raw = (sub + subOct + harm2) * attack * decay * velocity * 1.25;
    return Math.tanh(raw) * 0.7;
  }, 0.5, 0.5);
}

// 4. GLASSY MARIMBA ARPEGGIO
function renderMarimba(startSample, freq, pan = 0.5) {
  const dur = Math.floor(SAMPLE_RATE * 0.55);
  const volL = (1 - pan) * 0.22;
  const volR = pan * 0.22;
  mix(startSample, dur, (t) => {
    const f1 = Math.sin(2 * Math.PI * freq * t);
    const f2 = Math.sin(2 * Math.PI * freq * 2.76 * t) * 0.22;
    const f3 = Math.sin(2 * Math.PI * freq * 5.4 * t) * 0.08;
    const env = Math.exp(-t * 7.5);
    return (f1 + f2 + f3) * env * 0.9;
  }, volL, volR);
}

// ARRANGE FULL 48 SECONDS (88 BEATS)
const totalBeats = Math.floor(DURATION_SECS / BEAT_SECS); // 88 beats

console.log(`Synthesizing fresh 48.0s track: ${BPM} BPM, total beats: ${totalBeats}...`);

for (let b = 0; b < totalBeats; b++) {
  const beatStartSample = Math.floor(b * SAMPLE_PER_BEAT);
  const barIndex = Math.floor(b / 4);
  const beatInBar = b % 4;
  const chord = CHORDS[barIndex % CHORDS.length];

  // 1. CHORDS
  if (beatInBar === 0) {
    const dur = b >= 8 ? 2.5 : 3.8;
    const vel = b < 8 ? 0.38 : 0.48;
    renderRhodesChord(beatStartSample, dur, chord.notes, vel);
  } else if (beatInBar === 2 && b >= 8) {
    const syncStart = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
    renderRhodesChord(syncStart, 1.4, chord.notes, 0.42);
  }

  // 2. BASS (Enters on beat 4 / Bar 1 beat 4)
  if (b >= 4) {
    if (beatInBar === 0) {
      renderBass(beatStartSample, 1.4, chord.root, 0.6);
    } else if (beatInBar === 2) {
      const walkFreq = chord.root * 1.5;
      renderBass(beatStartSample, 0.9, walkFreq, 0.5);
    } else if (beatInBar === 3 && barIndex % 2 === 1) {
      const syncStart = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
      renderBass(syncStart, 0.45, chord.root * 1.33, 0.45);
    }
  }

  // 3. DRUMS
  if (b >= 8) {
    // Kick on beat 0 and beat 2
    if (beatInBar === 0) {
      renderKick(beatStartSample);
    } else if (beatInBar === 2) {
      renderKick(beatStartSample);
    } else if (beatInBar === 3 && barIndex % 2 === 0) {
      const syncKick = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
      renderKick(syncKick);
    }

    // Wood snap on beat 1 and beat 3 (beats 2 & 4 musical)
    if (beatInBar === 1 || beatInBar === 3) {
      renderWoodSnap(beatStartSample);
    }

    // Hi-hats
    renderHiHat(beatStartSample, true);
    const offbeat = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
    renderHiHat(offbeat, false);

    // Shakers (b >= 24)
    if (b >= 24 && b < 80) {
      for (let s = 0; s < 4; s++) {
        const shk = Math.floor((b + s * 0.25) * SAMPLE_PER_BEAT);
        renderShaker(shk);
      }
    }
  } else if (b >= 4 && b < 8) {
    // Gentle intro kick & hat
    if (beatInBar === 0 || beatInBar === 2) {
      renderKick(beatStartSample);
    }
    if (beatInBar === 1 || beatInBar === 3) {
      renderHiHat(beatStartSample, false);
    }
  }

  // 4. GLASSY MARIMBA ARPEGGIOS (Slide 4 MCP & Slide 5 Pipeline: b=36 to b=76)
  if (b >= 36 && b < 78) {
    const arpeggioPattern = [
      { beatOffset: 0.5, note: N.Eb5, pan: 0.35 },
      { beatOffset: 1.5, note: N.G5, pan: 0.65 },
      { beatOffset: 2.25, note: N.Bb5, pan: 0.4 },
      { beatOffset: 3.5, note: N.C6, pan: 0.6 },
    ];
    if (barIndex % 2 === 0 && beatInBar === 0) {
      arpeggioPattern.forEach((m) => {
        const pSample = Math.floor((b + m.beatOffset) * SAMPLE_PER_BEAT);
        if (pSample < TOTAL_SAMPLES) {
          renderMarimba(pSample, m.note, m.pan);
        }
      });
    }
  }
}

// Master fade in (0.25s) and musical fade out (1.8s at end)
const FADE_IN = Math.floor(SAMPLE_RATE * 0.25);
const FADE_OUT_START = Math.floor(SAMPLE_RATE * 46.2);
for (let i = 0; i < TOTAL_SAMPLES; i++) {
  let gain = 1.0;
  if (i < FADE_IN) {
    gain = i / FADE_IN;
  } else if (i >= FADE_OUT_START) {
    gain = 1.0 - (i - FADE_OUT_START) / (TOTAL_SAMPLES - FADE_OUT_START);
  }
  left[i] *= gain;
  right[i] *= gain;
}

// Soft-knee analog saturation & normalize to -0.5 dBFS
let maxPeak = 0;
for (let i = 0; i < TOTAL_SAMPLES; i++) {
  maxPeak = Math.max(maxPeak, Math.abs(left[i]), Math.abs(right[i]));
}
console.log(`Peak before mastering: ${maxPeak.toFixed(3)}`);
const normFactor = maxPeak > 0 ? 0.94 / maxPeak : 1.0;

const pcmBytes = new Int16Array(TOTAL_SAMPLES * 2);
for (let i = 0; i < TOTAL_SAMPLES; i++) {
  const l = Math.tanh(left[i] * normFactor) * 32767;
  const r = Math.tanh(right[i] * normFactor) * 32767;
  pcmBytes[i * 2] = Math.max(-32768, Math.min(32767, Math.floor(l)));
  pcmBytes[i * 2 + 1] = Math.max(-32768, Math.min(32767, Math.floor(r)));
}

function createWavHeader(numSamples, sampleRate = 44100, numChannels = 2, bitsPerSample = 16) {
  const byteRate = sampleRate * numChannels * (bitsPerSample / 8);
  const blockAlign = numChannels * (bitsPerSample / 8);
  const dataSize = numSamples * numChannels * (bitsPerSample / 8);
  const buffer = Buffer.alloc(44);

  buffer.write('RIFF', 0);
  buffer.writeUInt32LE(36 + dataSize, 4);
  buffer.write('WAVE', 8);
  buffer.write('fmt ', 12);
  buffer.writeUInt32LE(16, 16);
  buffer.writeUInt16LE(1, 20);
  buffer.writeUInt16LE(numChannels, 22);
  buffer.writeUInt32LE(sampleRate, 24);
  buffer.writeUInt32LE(byteRate, 28);
  buffer.writeUInt16LE(blockAlign, 32);
  buffer.writeUInt16LE(bitsPerSample, 34);
  buffer.write('data', 36);
  buffer.writeUInt32LE(dataSize, 40);

  return buffer;
}

const header = createWavHeader(TOTAL_SAMPLES, SAMPLE_RATE, 2, 16);
const pcmBuffer = Buffer.from(pcmBytes.buffer);
const finalWav = Buffer.concat([header, pcmBuffer]);

const outPath1 = path.resolve(__dirname, '../public/audio/music_modern.wav');
const outPath2 = path.resolve(__dirname, '../public/audio/music.wav');
fs.writeFileSync(outPath1, finalWav);
fs.writeFileSync(outPath2, finalWav);
console.log(`Saved brand-new 48.0s track to: ${outPath1} (${(finalWav.length / 1024 / 1024).toFixed(2)} MB)`);
