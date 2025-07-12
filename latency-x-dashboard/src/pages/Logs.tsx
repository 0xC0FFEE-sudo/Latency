import { useWebSocket } from "@/hooks/useWebSocket";

export function LogsPage() {
    const { ticks } = useWebSocket('ws://localhost:3000/ws');

    return (
        <div className="w-full">
            <h1 className="text-2xl font-bold mb-4">Live Data Logs</h1>
            <div className="bg-muted p-4 rounded-md font-mono text-xs overflow-auto h-96">
                {ticks.map((tick, index) => (
                    <div key={index}>{JSON.stringify(tick)}</div>
                ))}
            </div>
        </div>
    )
} 