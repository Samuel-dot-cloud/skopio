import { create } from "zustand";
import { TimeRangePreset } from "../../../types/tauri.gen";

interface DashboardFilterState {
  preset: TimeRangePreset;
  setPreset: (preset: TimeRangePreset) => void;
}

export const useDashboardFilter = create<DashboardFilterState>((set) => ({
  preset: "Today",
  setPreset: (preset) => set({ preset }),
}));
