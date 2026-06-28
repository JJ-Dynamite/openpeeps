"use client";
import { useState } from "react";
export default function Home() {
  const [illustrations] = useState([
    { id: "1", name: "Person Working", category: "business", options: ["skin_tone", "hair_color", "clothing"] },
    { id: "2", name: "Person Thinking", category: "people", options: ["skin_tone", "hair_style"] },
  ]);
  const [selected, setSelected] = useState<any>(null);
  return (
    <main className="min-h-screen bg-gradient-to-br from-lime-900 via-black to-green-900 text-white p-8">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-lime-400 to-green-400 bg-clip-text text-transparent">openpeeps</h1>
        <p className="text-xl text-gray-300 mb-8">Free hand-drawn illustrations</p>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {illustrations.map((ill) => (
            <div key={ill.id} onClick={() => setSelected(ill)}
              className="bg-white/10 backdrop-blur rounded-xl overflow-hidden hover:scale-105 transition cursor-pointer">
              <div className="aspect-square bg-white/20 flex items-center justify-center text-5xl">🧑‍🎨</div>
              <div className="p-3">
                <p className="font-semibold text-sm">{ill.name}</p>
                <p className="text-xs text-gray-400">{ill.category}</p>
              </div>
            </div>
          ))}
        </div>
        {selected && (
          <div className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-8" onClick={() => setSelected(null)}>
            <div className="bg-white/10 backdrop-blur rounded-2xl p-8 max-w-md w-full" onClick={(e) => e.stopPropagation()}>
              <h2 className="text-2xl font-bold mb-4">{selected.name}</h2>
              <div className="space-y-3">
                {selected.options.map((opt: string) => (
                  <div key={opt}>
                    <label className="text-sm text-gray-400 capitalize">{opt.replace("_", " ")}</label>
                    <div className="flex gap-2 mt-1">
                      {["#f5d0a9", "#8d5524", "#c68642", "#e0ac69"].map((c) => (
                        <div key={c} className="w-8 h-8 rounded-full border-2 border-white/30 cursor-pointer hover:border-white" style={{ backgroundColor: c }} />
                      ))}
                    </div>
                  </div>
                ))}
              </div>
              <button onClick={() => setSelected(null)} className="mt-6 px-6 py-2 bg-lime-600 rounded-full">Close</button>
            </div>
          </div>
        )}
      </div>
    </main>
  );
}