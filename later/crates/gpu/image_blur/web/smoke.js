// Tests the staged Rust benchmark's WGSL in a browser WebGPU implementation.
const result = document.querySelector("#result");
const width = 130;
const weights = [1, 4, 6, 4, 1];

/** Builds an independent CPU reference by applying the full 5-by-5 filter. */
function referenceAt(input, x, y) {
  let sum = 0;
  for (let dy = 0; dy < weights.length; dy++) {
    const row = Math.min(width - 1, Math.max(0, y + dy - 2));
    for (let dx = 0; dx < weights.length; dx++) {
      const column = Math.min(width - 1, Math.max(0, x + dx - 2));
      sum += weights[dy] * weights[dx] * input[row * width + column];
    }
  }
  return Math.floor((sum + 128) / 256);
}

/** Dispatches both compute passes and verifies every returned pixel. */
async function verifyBlur() {
  if (!navigator.gpu) throw new Error("WebGPU requires a supported browser and a secure context");
  const adapter = await navigator.gpu.requestAdapter();
  if (!adapter) throw new Error("No WebGPU adapter was available");
  const device = await adapter.requestDevice();

  try {
    const response = await fetch("../src/blur.wgsl");
    if (!response.ok) throw new Error(`Could not load blur.wgsl (HTTP ${response.status})`);
    const module = device.createShaderModule({ code: await response.text() });
    const { messages } = await module.getCompilationInfo();
    const errors = messages.filter(({ type }) => type === "error");
    if (errors.length) throw new Error(errors.map(({ message }) => message).join("\n"));

    const input = Uint32Array.from({ length: width * width }, (_, index) => {
      const x = index % width;
      const y = Math.floor(index / width);
      return (13 * x + 19 * y + ((x ^ y) & 31) * 11) & 255;
    });
    const storage = GPUBufferUsage.STORAGE;
    const buffer = (usage) => device.createBuffer({ size: input.byteLength, usage });
    const source = buffer(storage | GPUBufferUsage.COPY_DST);
    const scratch = buffer(storage);
    const output = buffer(storage | GPUBufferUsage.COPY_SRC);
    const readback = buffer(GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ);
    const dimensions = device.createBuffer({
      size: 16,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    device.queue.writeBuffer(source, 0, input);
    device.queue.writeBuffer(dimensions, 0, new Uint32Array([width, width, 0, 0]));

    const encoder = device.createCommandEncoder();
    const groups = Math.ceil(width / 16);
    for (const [entryPoint, src, dst] of [
      ["horizontal", source, scratch],
      ["vertical", scratch, output],
    ]) {
      const pipeline = await device.createComputePipelineAsync({
        layout: "auto",
        compute: { module, entryPoint },
      });
      const bindings = device.createBindGroup({
        layout: pipeline.getBindGroupLayout(0),
        entries: [dimensions, src, dst].map((buffer, binding) => ({
          binding,
          resource: { buffer },
        })),
      });
      const pass = encoder.beginComputePass();
      pass.setPipeline(pipeline);
      pass.setBindGroup(0, bindings);
      pass.dispatchWorkgroups(groups, groups);
      pass.end();
    }

    encoder.copyBufferToBuffer(output, 0, readback, 0, input.byteLength);
    device.queue.submit([encoder.finish()]);
    await readback.mapAsync(GPUMapMode.READ);
    try {
      const actual = new Uint32Array(readback.getMappedRange());
      for (let i = 0; i < actual.length; i++) {
        const expected = referenceAt(input, i % width, Math.floor(i / width));
        if (actual[i] !== expected) {
          throw new Error(`Pixel ${i}: got ${actual[i]}, expected ${expected}`);
        }
      }
    } finally {
      readback.unmap();
    }
    const { vendor, architecture, description, isFallbackAdapter } = adapter.info;
    const name = description || [vendor, architecture].filter(Boolean).join(" ") || "undisclosed adapter";
    const adapterType = isFallbackAdapter ? "software fallback" : "WebGPU adapter";
    result.textContent = `Verified ${input.length} pixels on ${name} (${adapterType}).`;
    result.dataset.state = "passed";
  } finally {
    device.destroy();
  }
}

verifyBlur().catch((error) => {
  result.textContent = error.message;
  result.dataset.state = "failed";
});
