import { useState, useEffect, useRef } from 'react';

interface Tick {
    source: string;
    symbol: string;
    price: number;
    volume: number;
    received_at: string;
}

interface Trade {
    order_id: string;
    symbol: string;
    side: 'Buy' | 'Sell';
    price: number;
    quantity: number;
    timestamp: string;
}

interface LatencyUpdate {
    order_id: string;
    latency_us: number;
}

interface LogEntry {
    timestamp: string;
    level: string;
    message: string;
    target: string;
}

type DashboardEvent = 
    | { type: 'Tick', data: Tick }
    | { type: 'Trade', data: Trade }
    | { type: 'LatencyUpdate', data: LatencyUpdate }
    | { type: 'Log', data: LogEntry };

export type { Tick, Trade, LatencyUpdate, LogEntry, DashboardEvent };

export function useWebSocket(url: string) {
    const [ticks, setTicks] = useState<Tick[]>([]);
    const [trades, setTrades] = useState<Trade[]>([]);
    const [latencies, setLatencies] = useState<LatencyUpdate[]>([]);
    const [logs, setLogs] = useState<LogEntry[]>([]);
    const [isConnected, setIsConnected] = useState(false);

    useEffect(() => {
        const ws = new WebSocket(url);

        ws.onopen = () => {
            console.log('WebSocket connected');
            setIsConnected(true);
        };

        ws.onmessage = (event) => {
            const message: DashboardEvent = JSON.parse(event.data);
            switch (message.type) {
                case 'Tick':
                    setTicks(prev => [message.data, ...prev].slice(0, 100)); // Keep last 100 ticks
                    break;
                case 'Trade':
                    setTrades(prev => [message.data, ...prev].slice(0, 100));
                    break;
                case 'LatencyUpdate':
                    setLatencies(prev => [message.data, ...prev].slice(0, 100));
                    break;
                case 'Log':
                    setLogs(prev => [message.data, ...prev].slice(0, 100));
                    break;
            }
        };

        ws.onclose = () => {
            console.log('WebSocket disconnected');
            setIsConnected(false);
        };
        
        ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            setIsConnected(false);
        };

        return () => {
            ws.close();
        };
    }, [url]);

    return { ticks, trades, latencies, logs, isConnected };
} 