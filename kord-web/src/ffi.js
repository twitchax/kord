// Raw PCM capture returning { sampleRate, channels, data: Float32Array } (mono channel).
export async function recordMicrophone(seconds, frameSize = 1024) {
    // Check capabilities.

    if (typeof navigator === 'undefined' || !navigator.mediaDevices) {
        throw new Error('no media devices');
    }

    // Set up the stream and context.

    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    const AudioCtx = window.AudioContext || window.webkitAudioContext; // Safari fallback
    const ctx = new AudioCtx();

    const sampleRate = ctx.sampleRate;
    const targetFrames = Math.ceil(seconds * sampleRate);
    const source = ctx.createMediaStreamSource(stream);
    const proc = ctx.createScriptProcessor(frameSize, 1, 1);
    const data = new Float32Array(targetFrames);

    let offset = 0;
    let finished = false;

    function cleanup() {
        if (finished) return;
        finished = true;
        try { source.disconnect(); } catch { }
        try { proc.disconnect(); } catch { }
        try { stream.getTracks().forEach(t => t.stop()); } catch { }
        try { ctx.close(); } catch { }
    }

    return await new Promise((resolve, reject) => {
        proc.onaudioprocess = e => {
            if (finished) return;
            const input = e.inputBuffer.getChannelData(0);
            const remaining = targetFrames - offset;
            const copyCount = Math.min(remaining, input.length);
            data.set(input.subarray(0, copyCount), offset);
            offset += copyCount;
            if (offset >= targetFrames) {
                cleanup();
                resolve({ sampleRate, channels: 1, data });
            }
        };
        proc.onerror = e => { cleanup(); reject(e?.message || 'pcm error'); };
        source.connect(proc);
        proc.connect(ctx.destination);
        setTimeout(() => {
            if (!finished) {
                cleanup();
                if (offset > 0) {
                    resolve({ sampleRate, channels: 1, data: data.subarray(0, offset) });
                } else {
                    reject('timeout');
                }
            }
        }, (seconds * 1000) + 250);
    });
}
