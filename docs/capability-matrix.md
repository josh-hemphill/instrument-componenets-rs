# Capability matrix (IVI-inspired, not IVI-conformant)

Rows are **Base** (target before calling a class “release-ready”) or **Extension** (optional). Status: `done` / `partial` / `todo` / `deferred`.

## Dmm

| Group | Capability | Status |
|---|---|---|
| Base | Measure Vdc / Vac / Idc / Ω | done |
| Base | Configure range + resolution | done |
| Base | Immediate / software trigger + Read / Initiate+Fetch | done |
| Base | AC current | done |
| Base | 2-wire / 4-wire resistance | done |
| Extension | Temperature | done |
| Extension | Multipoint / sample count | deferred |

## DcPowerSupply

| Group | Capability | Status |
|---|---|---|
| Base | Set V / I limit, enable output, readback | done |
| Base | Query output state | done |
| Base | OVP set / enable / query | done |
| Base | Channel count helper | done |
| Extension | Remote sense | done |

## FunctionGenerator

| Group | Capability | Status |
|---|---|---|
| Base | Standard waveform, frequency, amplitude, offset, output | done |
| Base | Duty cycle, output load | done |
| Base | Burst + trigger (imm/ext/bus) | done |
| Extension | Arbitrary waveform | deferred |

## Oscilloscope

| Group | Capability | Status |
|---|---|---|
| Base | Timebase, channel scale, run/stop, ASCII trace | done |
| Base | Channel enable + coupling | done |
| Base | Edge trigger | done |
| Base | Single acquire | done |
| Base | Vpp / frequency measurement | done |
| Extension | Binary waveform | todo |

## Switch

| Group | Capability | Status |
|---|---|---|
| Base | Close / open / is_closed / open_all (matrix pair) | done |
| Base | Path naming helpers | done |
| Extension | Scan list | deferred |

## Counter

| Group | Capability | Status |
|---|---|---|
| Base | Frequency / period / totalize | done |
| Base | Gate time, channel select, timeout | partial |

## PowerMeter

| Group | Capability | Status |
|---|---|---|
| Base | Configure units / auto-range / avg / corr freq / offset | done |
| Base | Read / Initiate+Fetch | done |
| Extension | Dual channel, manual range, cal, ref osc | deferred |

## SpectrumAnalyzer

| Group | Capability | Status |
|---|---|---|
| Base | Center, span, RBW/VBW, ref level | done |
| Base | Trace fetch (ASCII), marker peak | done |
| Base | Single / continuous sweep + wait | done |
| Extension | ACLR / OBW / EMI detectors | deferred |
