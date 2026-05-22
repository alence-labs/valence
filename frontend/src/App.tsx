import { useEffect, useState } from 'react';
import { useStellarPasskey } from './hooks/useStellarPasskey';

interface Capacity {
  total_cpu_cores: number;
  total_gpu_units: number;
  total_storage_gb: number;
}

const initialCapacity: Capacity = {
  total_cpu_cores: 0,
  total_gpu_units: 0,
  total_storage_gb: 0,
};

function App() {
  const [capacity, setCapacity] = useState<Capacity>(initialCapacity);
  const { connectWithPasskey, connected, account } = useStellarPasskey();

  useEffect(() => {
    fetch('/capacity')
      .then((response) => response.json())
      .then(setCapacity)
      .catch(() => setCapacity(initialCapacity));
  }, []);

  return (
    <div className="min-h-screen bg-slate-950 px-6 py-10 text-slate-100">
      <div className="mx-auto max-w-6xl rounded-3xl border border-slate-800 bg-slate-900/95 p-8 shadow-card">
        <header className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
          <div>
            <p className="text-sm uppercase tracking-[0.3em] text-sky-300">VALENCE</p>
            <h1 className="mt-3 text-4xl font-semibold leading-tight text-white">DePIN Compute & Storage Orchestration</h1>
            <p className="mt-4 max-w-2xl text-slate-400">A production-grade dashboard scaffolded for Stellar Soroban with passkey-ready authentication and global capacity visualization.</p>
          </div>
          <button
            onClick={connectWithPasskey}
            className="inline-flex items-center rounded-full bg-brand-500 px-6 py-3 text-sm font-semibold text-white transition hover:bg-brand-600"
          >
            {connected ? `Connected: ${account}` : 'Connect with Passkey'}
          </button>
        </header>

        <section className="mt-10 grid gap-6 md:grid-cols-3">
          <div className="rounded-3xl border border-slate-800 bg-slate-950/80 p-6">
            <p className="text-sm uppercase tracking-[0.25em] text-slate-500">GPU Pool</p>
            <p className="mt-4 text-4xl font-semibold text-white">{capacity.total_gpu_units}</p>
            <p className="mt-2 text-slate-400">Global GPU units available across registered nodes.</p>
          </div>
          <div className="rounded-3xl border border-slate-800 bg-slate-950/80 p-6">
            <p className="text-sm uppercase tracking-[0.25em] text-slate-500">Storage Pool</p>
            <p className="mt-4 text-4xl font-semibold text-white">{capacity.total_storage_gb} GB</p>
            <p className="mt-2 text-slate-400">Total storage capacity actively registered in the network.</p>
          </div>
          <div className="rounded-3xl border border-slate-800 bg-slate-950/80 p-6">
            <p className="text-sm uppercase tracking-[0.25em] text-slate-500">CPU Pool</p>
            <p className="mt-4 text-4xl font-semibold text-white">{capacity.total_cpu_cores}</p>
            <p className="mt-2 text-slate-400">Available CPU cores for orchestration and compute dispatch.</p>
          </div>
        </section>

        <section className="mt-10 grid gap-6 lg:grid-cols-2">
          <article className="rounded-3xl border border-slate-800 bg-slate-950/80 p-6">
            <h2 className="text-xl font-semibold text-white">Network Health</h2>
            <p className="mt-3 text-slate-400">Valence is optimized for enterprise-grade node operator registration, native collateral staking, and SLA-driven reward flows.</p>
          </article>
          <article className="rounded-3xl border border-slate-800 bg-slate-950/80 p-6">
            <h2 className="text-xl font-semibold text-white">Stellar Integration</h2>
            <p className="mt-3 text-slate-400">Client-side hooks are prepared for @stellar/stellar-sdk integration and browser-native Passkey signing using secp256r1.</p>
          </article>
        </section>
      </div>
    </div>
  );
}

export default App;
