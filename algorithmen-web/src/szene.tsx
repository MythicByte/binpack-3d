import { useEffect, useState } from "react";
import { Canvas } from "@react-three/fiber";
import { CameraControls, Edges } from "@react-three/drei";
import  { BinSpec, ItemSpec, AlgorithmenFirstWasm } from "../../pkg/algorithmen_test3.js";

// ✅ Fix 3: use a plain interface so React state holds normal JS values
interface PlacedResult {
  x: number;
  y: number;
  z: number;
  size_x: number;
  size_y: number;
  size_z: number;
}
// ✅ store bin dimensions so JSX can access them
interface BinDims {
  x: number;
  y: number;
  z: number;
}

export default function Szene() {
  const [results, setResults] = useState<PlacedResult[]>([]);
  const [binDims, setBinDims] = useState<BinDims>({ x: 100, y: 100, z: 100 });


  useEffect(() => {
    (async () => {
      
      const bin = new BinSpec(100, 100, 100, 100000, 0);
        setBinDims({ x: bin.x, y: bin.y, z: bin.z });

      let items = [
        new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(20, 15, 10, 30, 2),
        new ItemSpec(5, 5, 5, 10, 3),
new ItemSpec(10, 10, 10, 50, 1),
        new ItemSpec(20, 15, 10, 30, 2),new ItemSpec(20, 15, 10, 30, 2),
      ];
      for (let index = 0; index < 100 ;index++) {
        items.push(new ItemSpec(20, 15, 10, 30, 2));
      }
      try {
        const algo = AlgorithmenFirstWasm.create(items, bin);
        console.log("Space left:", algo.space_left());
        const r = algo.calculate();

        // ✅ Fix 3: extract plain values from WASM objects immediately
        const mapped: PlacedResult[] = Array.from(r).map((item) => ({
          x: item.x,
          y: item.y,
          z: item.z,
          size_x: item.size_x,
          size_y: item.size_y,
          size_z: item.size_z,
        }));
        setResults(mapped);
      } catch (e) {
        console.error("WASM calculate failed:", e);
      }
    })();
  }, []);

  return (
    <div className="w-screen h-screen">
      <Canvas className="w-full h-full">
        <ambientLight intensity={0.4} />
        <directionalLight position={[3, 5, 2]} intensity={1} />
        {results.map((e, i) => (
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

        {/* ✅ bin outline — centered at half its own size */}
        <WireCube
          color="red"
          position={[binDims.x / 2, binDims.y / 2, binDims.z / 2]}
          size={[binDims.x, binDims.y, binDims.z]}
        />
        <CameraControls />
      </Canvas>
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
  // ✅ normalize: scalar → [n, n, n], array → use as-is
  const dims: [number, number, number] = Array.isArray(size)
    ? (size as [number, number, number])
    : [size as number, size as number, size as number];

  return (
    <mesh position={position}>
      <boxGeometry args={dims} /> {/* ✅ spread correctly */}
      <meshBasicMaterial color={color} transparent opacity={0} />
      <Edges color={color} />
    </mesh>
  );
}
