import { create } from "zustand";
import type { components } from "@/types/schema";

type CurrentUser = components["schemas"]["AuthResponse"]["user"];

interface AuthState {
	currentUser: CurrentUser | null;
	isReady: boolean;

	setCurrentUser: (user: CurrentUser) => void;
	setReady: () => void;
	logout: () => void;
	isLoggedIn: () => boolean;
}

export const useAuthStore = create<AuthState>((set, get) => ({
	currentUser: null,
	isReady: false,

	setCurrentUser: (user: CurrentUser) =>
		set({ currentUser: user, isReady: true }),
	getCurrentUser: () => get().currentUser,
	setReady: () => set({ isReady: true }),
	logout: () => set({ currentUser: null }),
	isLoggedIn: () => {
		return !!get().currentUser;
	},
}));
