import { DollarSign, ListOrdered, Zap } from "lucide-react";
import { useWebSocket } from "@/hooks/useWebSocket";
import { StatCard } from "@/components/dashboard/StatCard";
import { LatencyChart } from "@/components/dashboard/LatencyChart";
import { TradesTable } from "@/components/dashboard/TradesTable";

export function Overview() {
    const { isConnected, trades, latencies } = useWebSocket('ws://localhost:3000/ws');

    return (
        <div className="flex-1 space-y-4 p-8 pt-6">
            <div className="flex items-center justify-between space-y-2">
                <h2 className="text-3xl font-bold tracking-tight">Dashboard</h2>
                <div className="flex items-center space-x-2">
                    <div className={`h-3 w-3 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`}></div>
                    <span>{isConnected ? 'Connected' : 'Disconnected'}</span>
                </div>
            </div>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                <StatCard title="Total Revenue" content="$45,231.89" subtext="+20.1% from last month" icon={<DollarSign className="text-muted-foreground h-4 w-4"/>} />
                <StatCard title="Latency" content={`${latencies[0]?.latency_us ?? 0}µs`} subtext="+180.1% from last month" icon={<Zap className="text-muted-foreground h-4 w-4"/>} />
                <StatCard title="Total Trades" content={trades.length.toString()} subtext="+19% from last month" icon={<ListOrdered className="text-muted-foreground h-4 w-4"/>} />
            </div>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
                <LatencyChart data={latencies} />
                <TradesTable trades={trades} />
            </div>
        </div>
    )
} 