
import { NavLink } from "react-router-dom";
import { Settings, Home, LineChart, FileText } from "lucide-react";

export function Layout({ children }: { children: React.ReactNode }) {
    return (
        <div className="flex min-h-screen w-full flex-col bg-muted/40">
            <aside className="fixed inset-y-0 left-0 z-10 hidden w-60 flex-col border-r bg-background sm:flex">
                <nav className="flex flex-col items-start gap-4 px-4 py-4">
                    <h1 className="text-2xl font-bold self-center mb-4">Latency-X</h1>
                    <NavLink to="/" className={({ isActive }: { isActive: boolean }) => `flex w-full items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary ${isActive ? "text-primary bg-muted" : ""}`}>
                        <Home className="h-4 w-4" />
                        Overview
                    </NavLink>
                    <NavLink to="/trades" className={({ isActive }: { isActive: boolean }) => `flex w-full items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary ${isActive ? "text-primary bg-muted" : ""}`}>
                        <LineChart className="h-4 w-4" />
                        Trades
                    </NavLink>
                    <NavLink to="/logs" className={({ isActive }: { isActive: boolean }) => `flex w-full items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary ${isActive ? "text-primary bg-muted" : ""}`}>
                        <FileText className="h-4 w-4" />
                        Logs
                    </NavLink>
                    <NavLink to="/settings" className={({ isActive }: { isActive: boolean }) => `flex w-full items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary ${isActive ? "text-primary bg-muted" : ""}`}>
                        <Settings className="h-4 w-4" />
                        Settings
                    </NavLink>
                </nav>
            </aside>
            <div className="flex flex-col sm:gap-4 sm:py-4 sm:pl-60">
                <main className="grid flex-1 items-start gap-4 p-4 sm:px-6 sm:py-0 md:gap-8">
                    {children}
                </main>
            </div>
        </div>
    );
} 