import { useEffect, useState } from "react";
import { Canvas } from "@react-three/fiber";
import { CameraControls, Edges } from "@react-three/drei";
import { BinSpec, ItemSpec, AlgorithmenFirstWasm } from "../../pkg/algorithmen_test3.js";

interface PlacedResult {
  x: number;
  y: number;
  z: number;
  size_x: number;
  size_y: number;
  size_z: number;
}

interface BinDims {
  x: number;
  y: number;
  z: number;
}

export default function Szene() {
  const [results, setResults] = useState<PlacedResult[]>([]);
  const [binDims, setBinDims] = useState<BinDims>({ x: 100, y: 100, z: 100 });
  const [step, setStep] = useState(0); // ✅ how many items to show

  useEffect(() => {
    (async () => {
        console.log("Starting Application");
      const bin = new BinSpec(100, 100, 100, 100000, 0);
      setBinDims({ x: bin.x, y: bin.y, z: bin.z });

      let  items = [
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
        new ItemSpec(5, 5, 5, 10, 3),
      ];
      for (let index = 0;index < 1000 - 10;index++) {
        items.push(new ItemSpec(5, 5, 5, 10, 3));
      }

      try {
        const algo = AlgorithmenFirstWasm.create(items, bin);
        const r = algo.calculate();
        const mapped: PlacedResult[] = Array.from(r).map((item) => ({
          x: item.x,
          y: item.y,
          z: item.z,
          size_x: item.size_x,
          size_y: item.size_y,
          size_z: item.size_z,
        }));
        setResults(mapped);
        setStep(mapped.length); // ✅ start with all items visible
      } catch (e) {
        console.error("WASM calculate failed:", e);
      }
    })();
  }, []);

  const visible = results.slice(0, step); // ✅ only render items up to current step

  return (
    <div className="w-screen h-screen relative">
      <Canvas className="w-full h-full">
        <ambientLight intensity={0.4} />
        <directionalLight position={[3, 5, 2]} intensity={1} />

        {visible.map((e, i) => (
          <WireCube
            key={i}
            position={[
              e.x + e.size_x / 2,
              e.y + e.size_y / 2,
              e.z + e.size_z / 2,
            ]}
            size={[e.size_x, e.size_y, e.size_z]}
            color="green"
          />
        ))}

        <WireCube
          color="red"
          position={[binDims.x / 2, binDims.y / 2, binDims.z / 2]}
          size={[binDims.x, binDims.y, binDims.z]}
        />

        <CameraControls />
        <WireCube />
      </Canvas>

      {/* ✅ overlay UI — pointer-events-none on parent so canvas still gets mouse */}
      <div className="absolute bottom-6 left-1/2 -translate-x-1/2 w-2/3 pointer-events-none">
        <div className="bg-black/60 text-white rounded-xl px-6 py-4 pointer-events-auto">
          <div className="flex justify-between text-sm mb-2">
            <span>Items packed</span>
            <span>{step} / {results.length}</span>
          </div>
          <input
            type="range"
            min={0}
            max={results.length}
            value={step}
            onChange={(e) => setStep(Number(e.target.value))}
            className="w-full accent-green-400"
          />
          {/* ✅ optional step buttons */}
          <div className="flex gap-2 mt-3 justify-center">
            <button
              onClick={() => setStep(0)}
              className="px-3 py-1 rounded bg-white/10 hover:bg-white/20 text-sm"
            >⏮ Reset</button>
            <button
              onClick={() => setStep((s) => Math.max(0, s - 1))}
              className="px-3 py-1 rounded bg-white/10 hover:bg-white/20 text-sm"
            >← Prev</button>
            <button
              onClick={() => setStep((s) => Math.min(results.length, s + 1))}
              className="px-3 py-1 rounded bg-white/10 hover:bg-white/20 text-sm"
            >Next →</button>
            <button
              onClick={() => setStep(results.length)}
              className="px-3 py-1 rounded bg-white/10 hover:bg-white/20 text-sm"
            >⏭ All</button>
          </div>
        </div>
      </div>
    </div>
  );
}

function WireCube({
  position = [0, 0, 0] as [number, number, number],
  size = [1, 1, 1] as [number, number, number] | number,
  color = "black",
}: {
  position?: [number, number, number];
  size?: [number, number, number] | number;
  color?: string;
}) {
  const dims: [number, number, number] = Array.isArray(size)
    ? (size as [number, number, number])
    : [size as number, size as number, size as number];

  return (
    <mesh position={position}>
      <boxGeometry args={dims} />
      <meshBasicMaterial color={color} transparent opacity={0} />
      <Edges color={color} />
    </mesh>
  );
}