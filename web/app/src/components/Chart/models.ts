import { UTCTimestamp } from "lightweight-charts";

export interface Kline {
  open_datetime: Date;
  close_datetime: Date;
  open: number | null;
  high: number | null;
  low: number | null;
  close: number | null;
  base_volume: number;
  quote_volume: number;
  taker_base_volume: number;
  taker_quote_volume: number;
  trades_number: number;
}

export interface Time {
  time: UTCTimestamp;
}

export interface Volume {
  volume: number;
}

export interface OHLC {
  open: number | null;
  high: number | null;
  low: number | null;
  close: number | null;
}

export interface OHLCV extends OHLC, Volume {}
export interface OHLV extends Volume {
  open: number;
  high: number;
  low: number;
}

export interface TOHLC extends OHLC, Time {}
export interface TOHLCV extends OHLCV, Time {}
export interface TV extends Time, Volume {}

export interface Trade {
  datetime: Date;
  price: number;
  quantity: number;
}
