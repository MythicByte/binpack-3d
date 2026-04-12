import { useEffect,  useState } from "react";
import { Canvas } from "@react-three/fiber";
import { CameraControls, Edges } from "@react-three/drei";
import * as wasm from "../../pkg/algorithmen_test3.js";

interface PlacedResult {
  id:number,
  x: number;
  y: number;
  z: number;
  size_x: number;
  size_y: number;
  size_z: number;
}

interface BinDims {
  id:number,
  x: number;
  y: number;
  z: number;
}

function getNumberFieldOrGetter(obj: any, key: string): number {
  const v = obj?.[key];
  // getter method style: bin.x()
  if (typeof v === "function") return Number(v.call(obj));
  // property getter style: bin.x
  return Number(v);
}

export default function Szene() {
  const [results, setResults] = useState<PlacedResult[]>([]);
  const [binDims, setBinDims] = useState<BinDims>({id:1, x: 100, y: 100, z: 100 });
  const [step, setStep] = useState(0);
  const [err, setErr] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    const  hasRun = {current:false};

    (async () => {
      if (hasRun.current) {
        return;
      }
      hasRun.current = true
      try {
        // optional init (some wasm-pack builds need it, some don't)
        const maybeInit = (wasm as any).default;
        if (typeof maybeInit === "function") await maybeInit();

        const BinSpec = (wasm as any).BinSpec;
        const ItemSpec = (wasm as any).ItemSpec;
        const Algo = (wasm as any).AlgorithmenFirstWasm;

        const bin = new BinSpec(1,100+ Math.floor(Math.random() * 1000), 100 + Math.floor(Math.random() * 1000), 200 + Math.floor(Math.random() * 1000), 100000, 0);

        const dims = {
          id: getNumberFieldOrGetter(bin,"id"),
          x: getNumberFieldOrGetter(bin, "x"),
          y: getNumberFieldOrGetter(bin, "y"),
          z: getNumberFieldOrGetter(bin, "z"),
        };
        if (!cancelled) setBinDims(dims);

        const items: any[] = [
        ];
        for (let i = 0; i < 1000; i++) {
          items.push(new ItemSpec(i+2,Math.floor(Math.random() * 100)+ 1,1+ Math.floor(Math.random() * 100),1+ Math.floor(Math.random() * 100), 10, Math.floor(Math.random() * 100)));
            //items.push(new ItemSpec(10,10,10,10,1));
        }

        const algo = Algo.create(items, bin);
        const r = algo.calculate() as any[];
        console.log("results returned:", r.length);
        console.log("removed:", r[0].ignore); // ignore field carries removed count

        const mapped: PlacedResult[] = r.map((it) => ({
          id: it.id,
          x: it.x,
          y: it.y,
          z: it.z,
          size_x: it.size_x,
          size_y: it.size_y,
          size_z: it.size_z,
        }));

        if (!cancelled) {
          setResults(mapped);
          setStep(mapped.length);
          setErr(null);
        }
      } catch (e: any) {
        console.error("WASM failed:", e);
        if (!cancelled) setErr(String(e?.message ?? e));
      }
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  const visible = results.slice(0, step);

  return (
    <div className="w-screen h-screen relative">
      {err && (
        <div className="absolute top-4 left-4 z-50 bg-red-600 text-white p-3 rounded">
          <div className="font-semibold">Error</div>
          <pre className="text-xs whitespace-pre-wrap">{err}</pre>
        </div>
      )}

      <Canvas className="w-full h-full" camera={{ fov: 60, near: 0.1, far: 500_000 }}>
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
            opacity={0}
          />
        ))}

        <WireCube
          color="red"
          position={[binDims.x / 2, binDims.y / 2, binDims.z / 2]}
          size={[binDims.x, binDims.y, binDims.z]}
        />

        <CameraControls />
      </Canvas>

      <div className="absolute bottom-6 left-1/2 -translate-x-1/2 w-2/3 pointer-events-none">
        <div className="bg-black/60 text-white rounded-xl px-6 py-4 pointer-events-auto">
          <div className="flex justify-between text-sm mb-2">
            <span>Items packed</span>
            <span>
              {step} / {results.length}
            </span>
          </div>
          <input
            type="range"
            min={0}
            max={results.length}
            value={step}
            onChange={(e) => setStep(Number(e.target.value))}
            className="w-full accent-green-400"
          />
        </div>
      </div>
    </div>
  );
}

function WireCube({
  position = [0, 0, 0] as [number, number, number],
  size = [1, 1, 1] as [number, number, number] | number,
  color = "black",
  opacity = 0,
}: {
  position?: [number, number, number];
  size?: [number, number, number] | number;
  color?: string;
  opacity?: number;
}) {
  const dims: [number, number, number] = Array.isArray(size)
    ? (size as [number, number, number])
    : [size as number, size as number, size as number];

  return (
    <mesh position={position}>
      <boxGeometry args={dims} />
      <meshBasicMaterial
        color={color}
        transparent={opacity < 1}
        opacity={opacity}
      />
      <Edges color={color} />
    </mesh>
  );
}
