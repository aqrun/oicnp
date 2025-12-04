'use client';

import { useState, useEffect, useMemo, useCallback } from 'react';
import {
  Week,
  SolarMonth,
} from '../vendor/tyme';
import {
  Container,
  MonthLabel,
  WeekList,
  DayItem,
  DayContent,
  DayNumber,
  DayText,
  DayBadge,
  NavButton,
} from './index.styled';

interface WeekHead {
  isWeekend: boolean;
  name: string;
}

interface DayData {
  day: number;
  holiday: { isWork: boolean } | null;
  isCurrentMonth: boolean;
  isToday: boolean;
  isWeekend: boolean;
  text: string;
  moon: boolean;
  moonIndex: number;
}

interface WeekData {
  days: DayData[];
}

interface MonthData {
  name: string;
  weeks: WeekData[];
}

interface SolarMonthCalendarProps {
  weekStart?: number; // 周开始日期，1 = 周一，默认 1
  className?: string;
  showSixtyCycle?: boolean; // 是否显示干支（如"壬申"），默认 false
}

export default function SolarMonthCalendar({
  weekStart = 1,
  className = '',
  showSixtyCycle = false,
}: SolarMonthCalendarProps): JSX.Element {
  const [month, setMonth] = useState<SolarMonth | null>(null);
  const [monthData, setMonthData] = useState<MonthData>({
    name: '',
    weeks: [],
  });

  // 初始化星期表头数据
  const weekHeads = useMemo<WeekHead[]>(() => {
    const heads: WeekHead[] = [];
    let w = Week.fromIndex(weekStart);
    for (let i = 0; i < 7; i++) {
      const weekName = w.getName();
      heads.push({
        isWeekend: w.getIndex() === 6 || w.getIndex() === 0,
        name: '周' + weekName, // 将"一"、"二"等改为"周一"、"周二"
      });
      w = w.next(1);
    }
    return heads;
  }, [weekStart]);

  // 初始化当前月份
  useEffect(() => {
    const now = new Date();
    const currentMonth = SolarMonth.fromYm(
      now.getFullYear(),
      now.getMonth() + 1,
    );
    setMonth(currentMonth.next(0));
  }, []);

  // 计算月份数据
  const compute = useCallback(() => {
    if (!month) {
      return;
    }

    const now = new Date();
    const currentMonth = SolarMonth.fromYm(
      now.getFullYear(),
      now.getMonth() + 1,
    );

    const monthName = month.toString();
    const weeks: WeekData[] = [];
    const monthWeeks = month.getWeeks(weekStart);

    for (let i = 0; i < monthWeeks.length; i++) {
      const days: DayData[] = [];
      const weekDays = monthWeeks[i].getDays();

      for (let x = 0; x < weekDays.length; x++) {
        const solarDay = weekDays[x];
        const lunarDay = solarDay.getLunarDay();
        const holiday = solarDay.getLegalHoliday();
        const weekIndex = solarDay.getWeek().getIndex();
        let weekend = weekIndex === 6 || weekIndex === 0;

        if (holiday && holiday.isWork()) {
          weekend = false;
        }

        // 计算显示文本（农历、节气等）
        let text: string | null = null;

        const solarFestival = solarDay.getFestival();
        if (solarFestival) {
          text = solarFestival.getName();
        }

        const lunarFestival = lunarDay.getFestival();
        if (lunarFestival) {
          text = lunarFestival.getName();
        }

        if (lunarDay.getDay() === 1) {
          const lunarMonth = lunarDay.getLunarMonth();
          text = lunarMonth.getName();
          if (lunarMonth.getMonthWithLeap() === 1) {
            text =
              lunarMonth.getLunarYear().getSixtyCycle().getName() +
              '年' +
              text;
          }
        }

        const jq = solarDay.getTerm();
        if (jq && jq.getSolarDay().equals(solarDay)) {
          text = jq.getName();
          if (jq.isJie() && showSixtyCycle) {
            text += ' ' + lunarDay.getMonthSixtyCycle() + '月';
          }
        }

        if (!text) {
          text = lunarDay.getName();
          if (showSixtyCycle) {
            text += ' ' + lunarDay.getSixtyCycle();
          }
        }

        const phaseDay = solarDay.getPhaseDay();

        days.push({
          day: solarDay.getDay(),
          holiday: holiday ? { isWork: holiday.isWork() } : null,
          isCurrentMonth: solarDay.getSolarMonth().equals(month),
          isToday:
            solarDay.getDay() === now.getDate() &&
            solarDay.getSolarMonth().equals(currentMonth),
          isWeekend: weekend,
          text: text,
          moon: phaseDay.getDayIndex() === 0,
          moonIndex: phaseDay.getPhase().getIndex(),
        });
      }

      weeks.push({
        days: days,
      });
    }

    setMonthData({
      name: monthName,
      weeks: weeks,
    });
  }, [month, weekStart, showSixtyCycle]);

  // 当月份改变时重新计算
  useEffect(() => {
    compute();
  }, [compute]);

  // 切换到上一个月
  const prevMonth = () => {
    if (!month) return;
    setMonth(month.next(-1));
  };

  // 切换到下一个月
  const nextMonth = () => {
    if (!month) return;
    setMonth(month.next(1));
  };

  // 只有当数据准备好后才显示日历内容，避免闪烁
  const isReady = monthData.weeks.length > 0;

  if (!isReady) {
    return (
      <Container className={className} style={{
        minHeight: '300px',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        color: '#999',
        border: '1px solid #e2e2e2',
        borderRadius: '8px',
        marginBottom: '16px',
      }}>
        <i className='iconfont icon-loading' style={{ fontSize: '36px' }} />
      </Container>
    ); // 数据未准备好时不渲染
  }

  return (
    <Container className={className}>
      <MonthLabel>{monthData.name}</MonthLabel>
      <WeekList className="week">
        {weekHeads.map((w, index) => (
          <DayItem
            key={index}
            $isWeekend={w.isWeekend}
            $isWeek={true}
          >
            {w.name}
          </DayItem>
        ))}
      </WeekList>
      {monthData.weeks.map((week, weekIndex) => (
        <WeekList key={weekIndex}>
          {week.days.map((d, dayIndex) => (
            <DayItem
              key={dayIndex}
              $isWeekend={d.isWeekend}
              $isHoliday={!!d.holiday && !d.holiday.isWork}
              $isHolidayWork={!!d.holiday && d.holiday.isWork}
              $isGray={!d.isCurrentMonth}
              $isToday={d.isToday}
            >
              <DayContent $hasMoon={d.moon}>
                <DayNumber>{d.day}</DayNumber>
                <DayText>{d.text}</DayText>
                {d.holiday ? (
                  <DayBadge>{d.holiday.isWork ? '班' : '休'}</DayBadge>
                ) : d.isToday ? (
                  <DayBadge>今</DayBadge>
                ) : null}
              </DayContent>
            </DayItem>
          ))}
        </WeekList>
      ))}
      <NavButton
        $direction="prev"
        onClick={prevMonth}
        title="上月"
        role="button"
        tabIndex={0}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            prevMonth();
          }
        }}
      />
      <NavButton
        $direction="next"
        onClick={nextMonth}
        title="下月"
        role="button"
        tabIndex={0}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            nextMonth();
          }
        }}
      />
    </Container>
  );
}

