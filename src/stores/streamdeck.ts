import { defineStore } from 'pinia'

type ProfileKeyConfig = {
    value?: number;
};

type Profile = {
    id: number;
    name: string;
    keys: Record<string, ProfileKeyConfig>;
};

type StreamdeckState = {
    currentProfileId: number;
    selectedElementId: string | null;
    profiles: Profile[];
    ledConfig: {
        effect: string;
        color: string;
        brightness: number;
        speed: number;
        size: number;
        tail: number;
        density: number;
        hue: number;
        hue_shift: number;
        saturation: number;
        spread: number;
    };
};

export const useStreamDeckStore = defineStore('streamdeck', {
    state: (): StreamdeckState => ({
        currentProfileId: 0,
        selectedElementId: null as string | null,

        profiles: [
            { id: 0, name: 'Main (Desktop)', keys: {} },
            { id: 1, name: 'Gaming', keys: {} },
            { id: 2, name: 'Streaming', keys: {} }
        ],

        // Globale LED Konfiguration (Standardwerte)
        ledConfig: {
            effect: 'ColorOrbit',
            color: '#00e5ff',
            brightness: 255,
            speed: 100,
            size: 4,
            tail: 80,
            density: 100,
            hue: 200,
            hue_shift: 50,
            saturation: 255,
            spread: 120
        }
    }),

    getters: {
        activeProfile: (state) => state.profiles.find(p => p.id === state.currentProfileId)
    },

    actions: {
        setProfile(id: number) {
            this.currentProfileId = id;
            this.selectedElementId = null;
        },
        selectElement(id: string | null) {
            // Toggle-Logik: Klick auf aktives Element hebt Auswahl auf
            this.selectedElementId = this.selectedElementId === id ? null : id;
        }
    }
})