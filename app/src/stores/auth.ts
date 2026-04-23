import { create } from "zustand";
import type { components } from "@/types/schema";
import { clearAuth } from "@/lib/auth-storage";

type CurrentUser = components["schemas"]["CurrentUserResponse"];

interface AuthState {
	currentUser: CurrentUser | null;
	isReady: boolean;
	setCurrentUser: (user: CurrentUser) => void;
	setReady: () => void;
	logout: () => Promise<void>;
}

export const useAuthStore = create<AuthState>((set) => ({
	currentUser: null,
	isReady: false,

	setCurrentUser: (user: CurrentUser) =>
		set({ currentUser: user, isReady: true }),
	setReady: () => set({ isReady: true }),

	logout: async () => {
		try {
			await clearAuth();
		} catch (error) {
			console.error("トークンの削除に失敗しました:", error);
		} finally {
			set({ currentUser: null });
		}
	},
}));

export const useIsLoggedIn = () => useAuthStore((state) => !!state.currentUser);
