// make-audio-v3-groove.mjs
// Generates a 30-second sophisticated, warm indie-electronic / lo-fi tech groove
// 106 BPM, Bbmaj7 - C - Dm7 - Am7 progression, warm Moog-style bass, felt piano chords,
// crisp organic kick & wooden rimshot, smooth syncopated marimba/pluck accents.
// Zero harsh FM chirps, zero goofy sounds - pure premium SaaS production.

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const SAMPLE_RATE = 44100;
const DURATION_SECS = 48.0;
const TOTAL_SAMPLES = Math.floor(SAMPLE_RATE * DURATION_SECS);
const BPM = 106;
const BEAT_SECS = 60 / BPM;
const SAMPLE_PER_BEAT = SAMPLE_RATE * BEAT_SECS;

// Stereo buffers
const left = new Float32Array(TOTAL_SAMPLES);
const right = new Float32Array(TOTAL_SAMPLES);

// Note frequencies (Hz)
const NOTES = {
  // Bass (octave 1-2)
  Bb1: 58.27, C2: 65.41, D2: 73.42, F2: 87.31, G2: 98.00, A2: 110.00,
  Bb2: 116.54, C3: 130.81, D3: 146.83, E3: 164.81, F3: 174.61, A3: 220.00,
  // Chords (octave 3-4)
  Bb3: 233.08, D4: 293.66, F4: 349.23, A4: 440.00, // Bbmaj7
  C4: 261.63, E4: 329.63, G4: 392.00,            // C
  D4: 293.66, F4: 349.23, A4: 440.00, C5: 523.25, // Dm7
  A3: 220.00, C4: 261.63, E4: 329.63, G4: 392.00, // Am7
  // Melodic accents (octave 4-5)
  D5: 587.33, E5: 659.25, F5: 698.46, G5: 783.99, A5: 880.00, C6: 1046.50
};

// Chord progression: 4 bars loop (each bar = 4 beats)
// Bar 1: Bbmaj7 (Bb3, D4, F4, A4)
// Bar 2: C (C4, E4, G4, C5)
// Bar 3: Dm7 (D4, F4, A4, C5)
// Bar 4: Am7 (A3, C4, E4, G4)
const CHORD_PROGRESSION = [
  { root: NOTES.Bb1, notes: [NOTES.Bb3, NOTES.D4, NOTES.F4, NOTES.A4] },
  { root: NOTES.C2,  notes: [NOTES.C4,  NOTES.E4, NOTES.G4, NOTES.C5] },
  { root: NOTES.D2,  notes: [NOTES.D4,  NOTES.F4, NOTES.A4, NOTES.C5] },
  { root: NOTES.A2,  notes: [NOTES.A3,  NOTES.C4, NOTES.E4, NOTES.G4] },
];

// Helper: Mix audio into stereo buffer
function mix(bufL, bufR, startSample, durSamples, generator, volL = 0.5, volR = 0.5) {
  const end = Math.min(TOTAL_SAMPLES, startSample + durSamples);
  for (let i = startSample; i < end; i++) {
    const t = (i - startSample) / SAMPLE_RATE;
    const s = generator(t, i - startSample);
    bufL[i] += s * volL;
    bufR[i] += s * volR;
  }
}

// 1. DRUMS SYNTHESIS
function renderKick(startSample) {
  const dur = Math.floor(SAMPLE_RATE * 0.35);
  mix(left, right, startSample, dur, (t) => {
    // Pitch envelope: drops fast from 130 Hz down to 48 Hz
    const pitch = 48 + 85 * Math.exp(-t * 28);
    const phase = 2 * Math.PI * (48 * t - (85 / 28) * Math.exp(-t * 28));
    // Amp envelope: snappy attack, round body
    const env = Math.exp(-t * 9.5);
    // Subtle click on transient
    const click = Math.exp(-t * 120) * (Math.random() * 2 - 1) * 0.18;
    return (Math.sin(phase) * env + click) * 0.85;
  }, 0.65, 0.65);
}

function renderRimshot(startSample) {
  const dur = Math.floor(SAMPLE_RATE * 0.22);
  mix(left, right, startSample, dur, (t) => {
    // Body resonance at 420 Hz & 860 Hz
    const body = (Math.sin(2 * Math.PI * 420 * t) * 0.5 + Math.sin(2 * Math.PI * 860 * t) * 0.3) * Math.exp(-t * 22);
    // Crisp wooden noise transient
    const noise = (Math.random() * 2 - 1) * Math.exp(-t * 38) * 0.5;
    return (body + noise) * 0.55;
  }, 0.48, 0.52);
}

function renderHiHat(startSample, accent = false) {
  const dur = Math.floor(SAMPLE_RATE * (accent ? 0.08 : 0.045));
  const amp = accent ? 0.24 : 0.14;
  const panL = 0.45;
  const panR = 0.55;
  mix(left, right, startSample, dur, (t) => {
    const decay = accent ? 35 : 55;
    const noise = (Math.random() * 2 - 1) * Math.exp(-t * decay);
    // Metallic tone
    const tone = Math.sin(2 * Math.PI * 6800 * t) * 0.15 * Math.exp(-t * decay);
    return (noise + tone) * amp;
  }, panL, panR);
}

function renderShaker(startSample) {
  const dur = Math.floor(SAMPLE_RATE * 0.05);
  mix(left, right, startSample, dur, (t) => {
    const noise = (Math.random() * 2 - 1) * Math.sin(Math.PI * (t / (dur / SAMPLE_RATE))) * 0.12;
    return noise;
  }, 0.55, 0.45);
}

// 2. CHORD SYNTHESIS (Warm Felt Piano / Pad Keys)
function renderKeysChord(startSample, durBeats, chordNotes, velocity = 0.4) {
  const durSamples = Math.floor(SAMPLE_PER_BEAT * durBeats);
  chordNotes.forEach((freq, idx) => {
    // Slight spread across stereo
    const panOffset = (idx - 1.5) * 0.15;
    const volL = (0.5 - panOffset) * velocity;
    const volR = (0.5 + panOffset) * velocity;

    mix(left, right, startSample, durSamples, (t) => {
      // Warm additive harmonics (Fundamental + 2nd + 3rd + soft 4th)
      const h1 = Math.sin(2 * Math.PI * freq * t);
      const h2 = Math.sin(2 * Math.PI * freq * 2 * t) * 0.4;
      const h3 = Math.sin(2 * Math.PI * freq * 3 * t) * 0.18;
      const h4 = Math.sin(2 * Math.PI * freq * 4 * t) * 0.06;
      // Soft gentle attack (15ms), smooth exponential decay
      const attack = Math.min(1, t / 0.015);
      const decay = Math.exp(-t * (1.1 / durBeats));
      // Subtle gentle chorus tremolo
      const trem = 1 + 0.05 * Math.sin(2 * Math.PI * 3.5 * t);
      return (h1 + h2 + h3 + h4) * attack * decay * trem * 0.28;
    }, volL, volR);
  });
}

// 3. WARM MOOG-STYLE BASSLINE
function renderBassNote(startSample, durBeats, freq, velocity = 0.5) {
  const durSamples = Math.floor(SAMPLE_PER_BEAT * durBeats);
  mix(left, right, startSample, durSamples, (t) => {
    // Sub oscillator (sine) + warm saturated saw/triangle
    const sub = Math.sin(2 * Math.PI * freq * t);
    const subOct = Math.sin(2 * Math.PI * (freq / 2) * t) * 0.35;
    // Saturated 2nd harmonic
    const harm2 = Math.sin(2 * Math.PI * freq * 2 * t) * 0.25;
    // Envelope
    const attack = Math.min(1, t / 0.008);
    const decay = Math.exp(-t * 2.2);
    // Subtle tube warmth saturation tanh approximation
    const raw = (sub + subOct + harm2) * attack * decay * velocity * 1.2;
    return Math.tanh(raw) * 0.65;
  }, 0.5, 0.5);
}

// 4. MELODIC GLASSY PLUCK ACCENTS
function renderPluck(startSample, freq, pan = 0.5) {
  const dur = Math.floor(SAMPLE_RATE * 0.6);
  const volL = (1 - pan) * 0.22;
  const volR = pan * 0.22;
  mix(left, right, startSample, dur, (t) => {
    // Bell / glassy pluck
    const f1 = Math.sin(2 * Math.PI * freq * t);
    const f2 = Math.sin(2 * Math.PI * freq * 2.756 * t) * 0.25;
    const f3 = Math.sin(2 * Math.PI * freq * 4.0 * t) * 0.1;
    const env = Math.exp(-t * 6.5);
    return (f1 + f2 + f3) * env;
  }, volL, volR);
}

// GENERATE ARRANGEMENT OVER 30 SECONDS
const totalBeats = Math.floor(DURATION_SECS / BEAT_SECS); // ~53 beats

console.log(`Generating 30s track: ${BPM} BPM, total beats: ${totalBeats}...`);

// Bar by bar loop
for (let b = 0; b < totalBeats; b++) {
  const beatStartSample = Math.floor(b * SAMPLE_PER_BEAT);
  const barIndex = Math.floor(b / 4);
  const beatInBar = b % 4;
  const chordInfo = CHORD_PROGRESSION[barIndex % CHORD_PROGRESSION.length];

  // 1. CHORDS
  // Chords play on beat 0 and syncopated on beat 2.5 (8th note before beat 3)
  if (beatInBar === 0) {
    const chordDur = b >= 4 ? 2.5 : 3.8;
    const vel = b < 4 ? 0.35 : 0.48;
    renderKeysChord(beatStartSample, chordDur, chordInfo.notes, vel);
  } else if (beatInBar === 2 && b >= 4) {
    // Syncopated push chord on beat 2.5
    const syncStart = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
    renderKeysChord(syncStart, 1.4, chordInfo.notes, 0.42);
  }

  // 2. BASS (enters on beat 4)
  if (b >= 4) {
    if (beatInBar === 0) {
      // Root note on beat 0
      renderBassNote(beatStartSample, 1.4, chordInfo.root, 0.58);
    } else if (beatInBar === 2) {
      // Walking note or fifth on beat 2
      const walkFreq = chordInfo.root * 1.5; // fifth
      renderBassNote(beatStartSample, 0.9, walkFreq, 0.48);
    } else if (beatInBar === 3 && (barIndex % 2 === 1)) {
      // Approach note into next chord
      const approachFreq = chordInfo.root * 1.33; // fourth
      const syncStart = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
      renderBassNote(syncStart, 0.45, approachFreq, 0.45);
    }
  }

  // 3. DRUMS (Intro is chill beats 0-7; full beat kicks in at b=8)
  if (b >= 8) {
    // Kick on beat 0, beat 2 (and syncopated beat 1.75 occasionally)
    if (beatInBar === 0) {
      renderKick(beatStartSample);
    } else if (beatInBar === 2) {
      renderKick(beatStartSample);
    } else if (beatInBar === 3 && (barIndex % 2 === 0)) {
      const syncKick = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
      renderKick(syncKick);
    }

    // Rimshot on beat 1 and beat 3 (standard 2 & 4 in musical counting)
    if (beatInBar === 1 || beatInBar === 3) {
      renderRimshot(beatStartSample);
    }

    // Hi-hats: 8th notes (on beat and off-beat)
    renderHiHat(beatStartSample, true);
    const offbeatSample = Math.floor((b + 0.5) * SAMPLE_PER_BEAT);
    renderHiHat(offbeatSample, false);

    // Shakers: 16th notes (b >= 16 during MCP & pipeline showcase)
    if (b >= 16) {
      for (let s = 0; s < 4; s++) {
        const shakerSample = Math.floor((b + s * 0.25) * SAMPLE_PER_BEAT);
        renderShaker(shakerSample);
      }
    }
  } else if (b >= 4 && b < 8) {
    // Light intro drums
    if (beatInBar === 0 || beatInBar === 2) {
      renderKick(beatStartSample);
    }
    if (beatInBar === 1 || beatInBar === 3) {
      renderHiHat(beatStartSample, false);
    }
  }

  // 4. MELODIC PLUCK MOTIF (enters in Scene 3 / beat 20 onwards)
  if (b >= 20 && b < 76) {
    // Elegant glassy melody accents
    const melodyPattern = [
      { beatOffset: 0.5, note: NOTES.D5, pan: 0.35 },
      { beatOffset: 1.5, note: NOTES.F5, pan: 0.65 },
      { beatOffset: 2.25, note: NOTES.A5, pan: 0.4 },
      { beatOffset: 3.5, note: NOTES.G5, pan: 0.6 },
    ];
    if (barIndex % 2 === 0 && beatInBar === 0) {
      melodyPattern.forEach(m => {
        const pSample = Math.floor((b + m.beatOffset) * SAMPLE_PER_BEAT);
        if (pSample < TOTAL_SAMPLES) {
          renderPluck(pSample, m.note, m.pan);
        }
      });
    }
  }
}

// Master fade in (0.3s) and fade out (1.8s at end)
const FADE_IN = Math.floor(SAMPLE_RATE * 0.3);
const FADE_OUT_START = Math.floor(SAMPLE_RATE * 46.0);
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

// Master limiter / soft-clip & convert to 16-bit PCM
console.log("Mastering audio (soft saturation & peak normalization)...");
let maxPeak = 0;
for (let i = 0; i < TOTAL_SAMPLES; i++) {
  maxPeak = Math.max(maxPeak, Math.abs(left[i]), Math.abs(right[i]));
}
console.log(`Peak before normalization: ${(maxPeak).toFixed(3)}`);
const normFactor = maxPeak > 0 ? 0.92 / maxPeak : 1.0;

const pcmBytes = new Int16Array(TOTAL_SAMPLES * 2);
for (let i = 0; i < TOTAL_SAMPLES; i++) {
  const l = Math.tanh(left[i] * normFactor) * 32767;
  const r = Math.tanh(right[i] * normFactor) * 32767;
  pcmBytes[i * 2] = Math.max(-32768, Math.min(32767, Math.floor(l)));
  pcmBytes[i * 2 + 1] = Math.max(-32768, Math.min(32767, Math.floor(r)));
}

// Build WAV header
function createWavHeader(numSamples, sampleRate = 44100, numChannels = 2, bitsPerSample = 16) {
  const byteRate = sampleRate * numChannels * (bitsPerSample / 8);
  const blockAlign = numChannels * (bitsPerSample / 8);
  const dataSize = numSamples * numChannels * (bitsPerSample / 8);
  const buffer = Buffer.alloc(44);

  buffer.write('RIFF', 0);
  buffer.writeUInt32LE(36 + dataSize, 4);
  buffer.write('WAVE', 8);
  buffer.write('fmt ', 12);
  buffer.writeUInt32LE(16, 16); // Subchunk1Size
  buffer.writeUInt16LE(1, 20);  // PCM format
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

const outPath = path.resolve(__dirname, '../public/audio/music_modern.wav');
fs.writeFileSync(outPath, finalWav);
console.log(`Saved new fresh modern audio track to: ${outPath} (${(finalWav.length / 1024 / 1024).toFixed(2)} MB)`);
