use anyhow::{Result};

use crate::app::exchange::binance;
// use crate::app::arh999;
use crate::exchange::Kline;
use ta::indicators::MovingAverageConvergenceDivergence as TaMACD;
use ta::Next;
use plotters::prelude::*;
use chrono::Local;
use crate::app::lark::lark_client;
use std::time::Instant;
use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;

static TREND_MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {    
    RwLock::new(HashMap::with_capacity(64))
});


fn get_trend(symbol: &str, interval: &str, trend: &str) -> String {
    let mut map = TREND_MAP.write().unwrap();
    // info!("get_trend symbol {} interval {} map {:?}", symbol, interval, map);
    let key = format!("{}_{}", symbol, interval);

    let mut return_msg = String::new();
    if let Some(prev_trend) = map.get(&key) {
        if prev_trend == trend {
            return_msg = format!("\n之前的 趋势变化是 {}", prev_trend);
        } else {
            return_msg = format!("\n之前的 趋势变化是 {},  <at user_id=\"all\">所有人</at>", prev_trend);
        }
    } else {
        return_msg = String::from("");
    }

    map.insert(key, trend.to_string());
    // info!("get_trend symbol {} interval {} map {:?}", symbol, interval, map);
    return_msg
}

fn check_macd_trend(histogram_line: &[f64]) -> String {

    let mut return_msg = String::new();
    let count = 20;
    let histogram_last = if histogram_line.len() <= count { histogram_line } else { &histogram_line[histogram_line.len() - count..] };

    // info!("{} {} check_klines_trend  {:?} ns", symbol, interval, klines_last);

    let len = histogram_last.len();
    let mut rise_count = 0;
    for i in (1..len).rev() {
        let cur = &histogram_last[i];
        let prev = &histogram_last[i-1];

        if cur > prev {
            rise_count += 1;
        } else {
            break;
        }
    }

    if rise_count > 0 {
        let msg =  format!("{} 根连续上升趋势", rise_count);
        return msg;
    }

    let mut fall_count = 0;
    for i in (1..len).rev() {
        let cur = &histogram_last[i];
        let prev = &histogram_last[i-1];

        if cur < prev {
            fall_count += 1;
        } else {
            break;
        }
    }

    if fall_count > 0 {
        let msg =  format!("{} 根连续下降趋势", fall_count);
        return msg;
    }
    return_msg
}

pub async fn find_crossover(interval : &str) -> Result<bool> {

    let formatted_date = Local::now().format("%Y-%m-%d").to_string();

    let symbols = vec![
        "BTCUSDT", "ETHUSDT",
        // "BTCUSDT",
    ];

    for symbol in symbols {
        match binance::get_klines_v3(&symbol, interval, 200).await {
            Ok(klines) => {
                let start = Instant::now();
                let (macd_line, signal_line, histogram_line) = calculate_macd(&klines);
                // info!("macd_line  {} {:?}", symbol, macd_line);
                // info!("signal_line  {} {:?}", symbol, signal_line);
                // info!("histogram_line  {} {:?}", symbol, histogram_line);

                let _ = check_klines_trend(&klines, &symbol, &interval).await?;

                let macd_chart_name = format!("macd_chart_{}_{}_{}", interval, symbol, formatted_date);
                let found = check_crossover(&histogram_line, &symbol, &interval).await?;

                // 获取第二个时间点
                let end = Instant::now();

                // 计算两个时间点之间的纳秒差
                let duration = end.duration_since(start);
                let nanoseconds = duration.as_nanos();

                info!("{} {} calc cost: {} ns", symbol, interval, nanoseconds);

                // if found {
                //     draw_chart(&macd_line, &signal_line, &histogram_line, &macd_chart_name)?;
                //     info!("{} done", macd_chart_name);
                // }
            },
            Err(e) => {
                error!("get_klines error, {} {} {}", symbol, interval, e)
            },
        };

    }
    Ok(true)
}

async fn check_klines_trend(klines: &[Kline], symbol: &str, interval: &str) -> Result<bool>  {
    let count = 10;
    let klines_last = if klines.len() <= count { klines } else { &klines[klines.len() - count..] };

    // info!("{} {} check_klines_trend  {:?} ns", symbol, interval, klines_last);

    let len = klines_last.len();
    let mut rise_count = 0;
    for i in (1..len).rev() {
        let cur = &klines_last[i];
        let prev = &klines_last[i-1];
        let cur_high: f64 = cur.2.parse().unwrap();
        let cur_low: f64 = cur.3.parse().unwrap();
        let prev_high: f64 = prev.2.parse().unwrap();
        let prev_low: f64 = prev.3.parse().unwrap();

        if cur_high > prev_high && cur_low > prev_low {
            rise_count += 1;
        } else {
            break;
        }
    }

    if rise_count > 0 {
        let msg =  format!("{} {} kline 出现连续上升趋势, {}", symbol, interval, rise_count);
        let ret_msg = lark_client::send_msg_by_interval(&msg, interval).await?;
        if ret_msg != "success" {
            error!("send_msg resp {}", ret_msg);
        }
        return Ok(true);
    }

    let mut fall_count = 0;
    for i in (1..len).rev() {
        let cur = &klines_last[i];
        let prev = &klines_last[i-1];
        let cur_high: f64 = cur.2.parse().unwrap();
        let cur_low: f64 = cur.3.parse().unwrap();
        let prev_high: f64 = prev.2.parse().unwrap();
        let prev_low: f64 = prev.3.parse().unwrap();

        if cur_high < prev_high && cur_low < prev_low {
            fall_count += 1;
        } else {
            break;
        }
    }

    if fall_count > 0 {
        let msg =  format!("{} {} kline 出现连续下降趋势, {}", symbol, interval, fall_count);
        let ret_msg = lark_client::send_msg_by_interval(&msg, interval).await?;
        if ret_msg != "success" {
            error!("send_msg resp {}", ret_msg);
        }
        return Ok(true);
    }
    return Ok(false);
}

async fn check_crossover(histogram_line: &[f64], symbol: &str, interval: &str) -> Result<bool> {
    let last_histogram = histogram_line[histogram_line.len() - 1];
    let prev_histogram = histogram_line[histogram_line.len() - 2];
    let prev3_histogram = histogram_line[histogram_line.len() - 3];
    let prev4_histogram = histogram_line[histogram_line.len() - 4];
    let prev5_histogram = histogram_line[histogram_line.len() - 5];

    let last_n = histogram_line[histogram_line.len() - 5..].to_vec();
    let mut msg = String::new();
    if (last_histogram > 0.0 && prev_histogram <= 0.0) || (prev3_histogram < 0.0 && prev_histogram >= 0.0) {
        msg =  format!("{} {} MACD 出现金叉, {:?} \n   *\n---\n*{}\n{}", symbol, interval, last_n, get_trend(symbol, interval, "出现金叉"), check_macd_trend(histogram_line));
    } else if (last_histogram < 0.0 && prev_histogram >= 0.0) || (prev3_histogram > 0.0 && prev_histogram <= 0.0) {
        msg =  format!("{} {} MACD 出现死叉, {:?} \n*  \n---\n   *{}\n{}", symbol, interval, last_n, get_trend(symbol, interval, "出现死叉"), check_macd_trend(histogram_line));
    }

    if msg.len() > 0 {
        let msg = lark_client::send_msg_by_interval(&msg, interval).await?;
        if msg != "success" {
            error!("send_msg resp {}", msg);
        }
        return Ok(true);
    }

    if last_histogram > prev_histogram && prev_histogram > prev3_histogram && prev3_histogram > prev4_histogram {
        // 说明 MACD 线在上升
        msg =  format!("{} {} MACD 线在上升, {:?} \n    *\n  **\n***{}\n{}", symbol, interval, last_n, get_trend(symbol, interval, "在上升"), check_macd_trend(histogram_line));
    } else if last_histogram < prev_histogram && prev_histogram < prev3_histogram && prev3_histogram < prev4_histogram {
        // 说明 MACD 线在下降
        msg =  format!("{} {} MACD 线在下降, {:?} \n*  \n** \n***{}\n{}", symbol, interval, last_n, get_trend(symbol, interval, "在下降"), check_macd_trend(histogram_line));
    }

    if msg.len() > 0 {
        let msg = lark_client::send_msg_by_interval(&msg, interval).await?;
        if msg != "success" {
            error!("send_msg resp {}", msg);
        }
        return Ok(true);
    }


    if prev_histogram > last_histogram && prev3_histogram > prev_histogram && prev3_histogram > prev4_histogram
        && prev4_histogram > prev5_histogram && prev5_histogram > 0.0{
        msg =  format!("{} {} MACD 线可能在顶部反转, {:?} \n   *  \n  *** \n*****{}\n{}", symbol, interval, last_n, get_trend(symbol, interval, "可能在顶部反转"), check_macd_trend(histogram_line));
    } else if prev_histogram < last_histogram && prev3_histogram < prev_histogram && prev3_histogram < prev4_histogram
        && prev4_histogram < prev5_histogram && prev3_histogram < 0.0 {
        msg =  format!("{} {} MACD 线可能在底部反转, {:?} \n*****\n  *** \n   *  {}\n{}", symbol, interval, last_n, get_trend(symbol, interval, "可能在底部反转"), check_macd_trend(histogram_line));
    }

    if msg.len() > 0 {
        let msg = lark_client::send_msg_by_interval(&msg, interval).await?;
        if msg != "success" {
            error!("send_msg resp {}", msg);
        }
        return Ok(true);
    }

    Ok(false)
}


fn draw_chart(macd_line: &[f64], signal_line: &[f64], histogram_line: &[f64], macd_chart_name: &str) -> Result<()> {

    let mut min = macd_line.iter().cloned().fold(f64::INFINITY, f64::min);
    let mut max = macd_line.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let min_h = histogram_line.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_h = histogram_line.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if min > min_h {
        min = min_h;
    }

    if max < max_h {
        max = max_h;
    }

    let file_name = format!("{}.png", macd_chart_name);
    // 创建图表
    let root = BitMapBackend::new(&file_name, (1600, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    // 设置图表标题和坐标轴
    let mut chart = ChartBuilder::on(&root)
        .caption(macd_chart_name, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..macd_line.len(), min..max)?;

    chart.configure_mesh()
        .x_labels(20)
        .y_labels(20)
        .x_label_formatter(&|x| format!("{}", x))
        .y_label_formatter(&|y| format!("{:.1}", y))
        .draw()?;

    // 绘制 MACD 线（蓝色）
    chart.draw_series(LineSeries::new(
        macd_line.iter().enumerate().map(|(x, y)| (x, *y)),
        &BLUE,
    ))?
    .label("MACD Line")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // 绘制 Signal 线（橙色）
    chart.draw_series(LineSeries::new(
        signal_line.iter().enumerate().map(|(x, y)| (x, *y)),
        &RED,
    ))?
    .label("Signal Line")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // 绘制 Histogram（绿色正数，红色负数）
    let bar_width = 0.8; // 柱状图宽度
    for (x, &h) in histogram_line.iter().enumerate() {
        let color = if h >= 0.0 { GREEN } else { RED };
        let x_start = x as f64 - bar_width / 2.0;
        let x_end = x as f64 + bar_width / 2.0;
        chart.draw_series(std::iter::once(Rectangle::new(
            [(x_start  as usize, 0.0), (x_end  as usize, h)],
            color.filled(),
        )))?;
    }

    // 添加图例
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn calculate_macd(klines: &[Kline]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let fast_period = 12;
    let slow_period = 26;
    let signal_period = 9;
    let mut macd = TaMACD::new(fast_period, slow_period, signal_period).unwrap();
    let mut macd_results = Vec::new();

    // 计算 MACD（需要足够的数据点）
    for kline in klines.iter() {
        let close: f64 = kline.4.parse().unwrap();
        let macd_result = macd.next(close);
        macd_results.push((macd_result.macd, macd_result.signal, macd_result.histogram));
    }

    // 提取 MACD 线、信号线和直方图
    let macd_line: Vec<f64> = macd_results.iter().map(|(m, _, _)| *m).collect();
    let signal_line: Vec<f64> = macd_results.iter().map(|(_, s, _)| *s).collect();
    let histogram_line: Vec<f64> = macd_results.iter().map(|(_, _, h)| *h).collect();

    // (macd_line, signal_line, histogram_line)

    // 取最后 n 个值
    let count = 100;
    let macd_line_last = if macd_line.len() <= count { macd_line } else { macd_line[macd_line.len() - count..].to_vec() };
    let signal_line_last = if signal_line.len() <= count { signal_line } else { signal_line[signal_line.len() - count..].to_vec() };
    let histogram_line_last = if histogram_line.len() <= count { histogram_line } else { histogram_line[histogram_line.len() - count..].to_vec() };

    (macd_line_last, signal_line_last, histogram_line_last)
}
