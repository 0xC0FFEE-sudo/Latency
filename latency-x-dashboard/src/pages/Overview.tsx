import { DollarSign, ListOrdered, Zap } from "lucide-react";
import { useWebSocket } from "@/hooks/useWebSocket";
import { StatCard } from "@/components/dashboard/StatCard";
import { LatencyChart } from "@/components/dashboard/LatencyChart";
import { TradesTable } from "@/components/dashboard/TradesTable";

export function Overview() {
    const { trades, latencies } = useWebSocket('ws://localhost:3000/ws');

    // Dummy P&L calculation for now
    const pnl = trades.reduce((acc, trade) => {
        return acc + (trade.side === 'Buy' ? -1 : 1) * trade.price * trade.quantity;
    }, 0);

    const averageLatency = latencies.length > 0
        ? latencies.reduce((acc, l) => acc + l.latency_us, 0) / latencies.length
        : 0;

    return (
        <div className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                <StatCard title="Total P&L (USD)" value={`$${pnl.toFixed(2)}`} icon={DollarSign} />
                <StatCard title="Total Trades" value={trades.length.toString()} icon={ListOrdered} />
                <StatCard title="Avg. Latency (µs)" value={averageLatency.toFixed(0)} icon={Zap} />
            </div>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
                <div className="lg:col-span-4">
                    <LatencyChart data={latencies} />
                </div>
                <div className="lg:col-span-3">
                    <TradesTable trades={trades} />
                </div>
            </div>
        </div>
    );
} 