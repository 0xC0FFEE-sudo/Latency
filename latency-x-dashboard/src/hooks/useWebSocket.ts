import { useState, useEffect } from 'react';

// This should match the Rust DashboardEvent enum
export interface Tick {
    source: string;
    symbol: string;
    price: number;
    volume: number;
    received_at: string;
}

export interface Trade {
    order_id: string;
    symbol: string;
    side: 'Buy' | 'Sell';
    price: number;
    quantity: number;
    timestamp: string;
}

export interface LatencyUpdate {
    order_id: string;
    latency_us: number;
}

export type DashboardEvent = 
    | { type: 'Tick', payload: Tick }
    | { type: 'Trade', payload: Trade }
    | { type: 'LatencyUpdate', payload: LatencyUpdate };


export function useWebSocket(url: string) {
    const [ticks, setTicks] = useState<Tick[]>([]);
    const [trades, setTrades] = useState<Trade[]>([]);
    const [latencies, setLatencies] = useState<LatencyUpdate[]>([]);
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
                    setTicks(prev => [message.payload, ...prev].slice(0, 100)); // Keep last 100 ticks
                    break;
                case 'Trade':
                    setTrades(prev => [message.payload, ...prev].slice(0, 100));
                    break;
                case 'LatencyUpdate':
                    setLatencies(prev => [message.payload, ...prev].slice(0, 100));
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

    return { ticks, trades, latencies, isConnected };
} 