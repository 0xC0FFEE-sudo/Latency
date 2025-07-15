import useLocalStorage from "use-local-storage";
import { Button } from "@/components/ui/button";
import { useEffect } from "react";

export function SettingsPage() {
    const [theme, setTheme] = useLocalStorage("theme", "light");

    useEffect(() => {
        document.documentElement.setAttribute('data-theme', theme);
    }, [theme]);

    const toggleTheme = () => {
        setTheme(theme === "light" ? "dark" : "light");
    };

    return (
        <div className="w-full">
            <h1 className="text-2xl font-bold mb-4">Settings</h1>
            <div className="space-y-4">
                <div className="flex items-center justify-between">
                    <p>Theme</p>
                    <Button onClick={toggleTheme}>
                        Switch to {theme === "light" ? "Dark" : "Light"} Mode
                    </Button>
                </div>
            </div>
        </div>
    );
} 