# Auditory Processing Module (APM) — Specification Sheet

## Overview
The Auditory Processing Module (APM) ingests structured medical scan captures and produces the fixed `3 x 12 x 12 x 3` Chromatic Tensor required by the Chromatic Cognition Core (CCC). The design preserves volumetric nuance while aligning with the CCC's tri-planar processing semantics that originated from the steganographic audio carrier. Each tensor channel retains a reversible mapping back to the originating scan metadata to support anomaly detection workflows.

## Priority 1 — Input and Structuring Schema

### Input Contract: Volumetric Medical Capture
* **Structure**: 4D tensor with axes `[Depth, Density, Direction, Intensity]`.
* **Depth (`D`)**: Number of contiguous voxels sampled along the anatomical sweep for the Region of Interest (ROI). The minimum contract expects `D >= 3` to guarantee one voxel per CCC chromatic layer.
* **Density (`ρ`)**: Normalized scalar sampled from the modality's attenuation curve (e.g., MRI signal, CT HU, PET uptake). Values must be linearly scaled into `[0, 1]` with modality-specific calibration metadata attached.
* **Direction (`→`)**: 3-component vector encoding acquisition orientation (azimuth, elevation, radial). Direction vectors are unit-normalized and aligned to the patient's coordinate frame.
* **Intensity (`I`)**: Multi-contrast array capturing modality-specific brightness or energy readings per voxel. Each entry maps to one of three contrast layers described in the Z-axis contract.

The ModalityMapper consumes this contract, replacing the previous text/code ingestion path. Any upstream adapter (e.g., DICOM loader) must project the raw capture into this schema before invoking the APM.

### Z-Axis Allocation — Medical Contrast Context
* **Layer Z0 — Primary Contrast**: Typically T1-weighted MRI or baseline CT attenuation. Serves as the anatomical anchor.
* **Layer Z1 — Secondary Contrast**: Complementary modality or timing, such as T2-weighted MRI or delayed-phase CT, emphasizing fluid or perfusion characteristics.
* **Layer Z2 — Derived/Filtered Channel**: Computed artifacts (e.g., subtraction, diffusion-weighted synthesis, spectral unmixing). This layer is optimized for highlighting subtle anomalies by applying the steganographic carrier filters originally tuned for audio watermark recovery.

Each contrast slice is time-synchronized and voxel-aligned. If a modality lacks a native third channel, the derived layer must be synthesized via the APM's filterbank (difference-of-Gaussians + phase-shift embedding) to retain anomaly sensitivity.

### Spatial Slice — 12 × 12 ROI Grid
* **ROI Selection**: Upstream localization isolates a `12 × 12` window representing the critical anatomical patch. Selection criteria include clinician-specified landmarks or automated saliency proposals.
* **Voxel Sampling**: Depth-aligned voxels are resampled via trilinear interpolation to the uniform `12 × 12` grid, ensuring isotropic spacing. Density and intensity values are averaged per cell, while direction vectors are preserved using spherical mean operations.
* **Boundary Handling**: When the ROI overlaps scan boundaries, mirror padding is applied before interpolation to avoid introducing zero-density artifacts.

### Chromatic Tensor Assembly — `3 × 12 × 12 × 3`
* **Axis Mapping**:
  * **Axis 0 (Contrast Layer)**: `[Z0, Z1, Z2]` as defined above.
  * **Axis 1-2 (Spatial Grid)**: `[Y, X]` indices spanning the `12 × 12` ROI.
  * **Axis 3 (Chromatic Channels)**: `[Carrier_Low, Carrier_Mid, Carrier_High]`, capturing the decomposed steganographic frequency bands.
* **Carrier Encoding**: For each voxel, the APM applies a triple-band filterbank mirroring the original auditory steganography pipeline:
  * **Low Band** emphasizes macro-pattern shifts (e.g., edema diffusion) using a 2D Hann-windowed average.
  * **Mid Band** encodes structural edges by applying a Scharr kernel and projecting onto the direction vector.
  * **High Band** isolates micro-texture anomalies through a phase-based residual, leveraging the derived channel for reinforcement.
* **Normalization**: Each carrier band is normalized to zero mean, unit variance per contrast layer to ensure downstream compatibility. Calibration metadata is appended to the tensor header for reproducibility.

## Diagnostic Fusion Considerations
* **Temporal Coherence**: If the input capture spans multiple timepoints, the Depth axis indexes them sequentially. The Z-axis layers must still represent contrast diversity; temporal variance is folded into the carrier normalization statistics.
* **Metadata Retention**: Alongside the tensor, the APM emits a manifest containing original voxel identifiers, calibration coefficients, and any synthetic layer transformations to guarantee traceability.
* **Anomaly Sensitivity**: The steganographic carrier decomposition guarantees that anomalies manifest as high-magnitude perturbations within the High Band of the derived layer. Downstream detectors leverage this property for improved true-positive rates.

## Implementation Notes
* The APM exposes a Rust struct `AuditoryProcessingModule` with an interface `fn process(input: MedicalCapture) -> ChromaticTensor`.
* `MedicalCapture` enforces the `[Depth, Density, Direction, Intensity]` contract; conversion utilities live in `src/modality/`.
* `ChromaticTensor` is a thin wrapper around `ndarray::Array4<f32>` tagged with contrast metadata.
* Unit tests must inject synthetic voxel grids verifying:
  * Proper Z-layer mapping for multiple contrast combinations.
  * Carrier normalization invariants (mean ≈ 0, variance ≈ 1).
  * Boundary mirroring correctness.

This specification anchors the medical-domain extension while preserving compatibility with the Chromatic Cognition Core.
