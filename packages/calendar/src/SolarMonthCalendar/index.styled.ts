'use client';

import styled, { css } from 'styled-components';

// 主容器
export const Container = styled.div`
  position: relative;
  margin-bottom: 16px;
  width: 100%;
  /* max-width: 36.75rem; 588px */
  clear: both;
  overflow: hidden;
  border-radius: 8px; /* 16px */
  padding: clamp(0.75rem, 2vw, 1.25rem); /* 12px - 20px，响应式 */
  /* box-shadow: 0 0.125rem 0.5rem 0 rgba(0, 0, 0, 0.08); */
  border: 1px solid #e2e2e2;
  font-size: 14px;
  min-height: 300px;

  &:hover {
    box-shadow: 0 0.125rem 0.5rem 0 rgba(0, 0, 0, 0.08);
  }
`;

// 月份背景文字
export const MonthLabel = styled.div`
  position: absolute;
  left: 0;
  top: 0;
  font-size: 55px; /* 48px - 80px，响应式 */
  width: 100%;
  height: 100%;
  text-align: center;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
  opacity: 0.1;
  pointer-events: none;
`;

// 星期列表
export const WeekList = styled.ul`
  display: block;
  clear: both;
  padding-top: 8px;

  &.week {
    padding-top: 0;
  }
`;

// 日期列表项
export const DayItem = styled.li<{
  $isWeekend?: boolean;
  $isHoliday?: boolean;
  $isHolidayWork?: boolean;
  $isGray?: boolean;
  $isToday?: boolean;
  $isWeek?: boolean;
}>`
  position: relative;
  list-style: none;
  float: left;
  display: block;
  width: calc(100% / 7); /* 自适应宽度，7列 */
  min-width: 0; /* 允许收缩 */
  /* height: ${(props) => (props.$isWeek ? '2rem' : 'clamp(2.5rem, 6vw, 3.375rem)')}; 响应式高度 */
  text-align: center;
  /* line-height: ${(props) => (props.$isWeek ? '2rem' : 'auto')}; */

  ${(props) =>
    props.$isWeekend &&
    css`
      color: #f11;
    `}

  ${(props) =>
    props.$isHoliday &&
    css`
      color: #eb3333;

      div {
        background: rgba(235, 51, 51, 0.05);
      }

      b,
      i {
        color: #eb3333;
      }

      u {
        background-color: #eb3333;
      }
    `}

  ${(props) =>
    props.$isHolidayWork &&
    css`
      div {
        background: rgba(120, 120, 120, 0.05);
      }

      b,
      i {
        color: rgba(30, 31, 36, 0.5);
      }

      u {
        background-color: #4e5877;
      }
    `}

  ${(props) =>
    props.$isGray &&
    css`
      opacity: 0.3;
    `}

  ${(props) =>
    props.$isToday &&
    css`
      div {
        background-color: #4e6ef2;
        color: #fff;
      }

      i {
        color: #fff;
      }

      u {
        background-color: #6b88ff;
      }

      b {
        color: #fff;
      }
    `}
`;

// 日期内容容器
export const DayContent = styled.div<{ $hasMoon?: boolean }>`
  position: relative;
  padding: 4px 0;
  margin: 0 2px;
  max-width: 5rem; /* 80px */
  border-radius: 4px; /* 10px */

  ${(props) =>
    props.$hasMoon &&
    css`
      /* 月相图标样式占位，后续可以添加背景图片 */
      background-repeat: no-repeat;
      background-position: 0.75rem 0.1875rem; /* 12px 3px */
      background-size: clamp(0.875rem, 2vw, 1.125rem); /* 14px - 18px，响应式 */
    `}

  @media (max-width: 768px) {
    border-radius: 0.5rem;
  }
`;

// 日期数字
export const DayNumber = styled.b`
  display: block;
  font-weight: normal;
  font-size: 16px;
  line-height: 1.2;
`;

// 日期文本（农历等）
export const DayText = styled.i`
  display: block;
  font-style: normal;
  font-size: 12px; /* 10px - 12px，响应式 */
  color: rgba(30, 31, 36, 0.5);
  white-space: nowrap;
`;

// 日期标签（休/班/今）
export const DayBadge = styled.u`
  position: absolute;
  right: -0.3125rem; /* -5px */
  top: -0.3125rem; /* -5px */
  background-color: #4e5877;
  color: #fff;
  font-size: 12px; /* 10px - 12px，响应式 */
  line-height: 0.875rem; /* 14px */
  padding: 0px 2px; /* 1px 2px */
  border-radius: 2px;
  text-decoration: none;
  white-space: nowrap;
`;

// 导航按钮
export const NavButton = styled.div<{ $direction: 'prev' | 'next' }>`
  position: absolute;
  top: 50%;
  ${(props) => (props.$direction === 'prev' ? 'left: 0;' : 'right: 0;')}
  width: 1.25rem; /* 20px */
  height: 1.25rem; /* 20px */
  margin-top: -0.625rem; /* -10px */
  font-size: 0;
  cursor: pointer;
  z-index: 1;
  transition: opacity 0.2s;

  &:hover {
    opacity: 0.7;
  }

  &::before {
    content: '';
    position: absolute;
    top: 0;
    ${(props) => (props.$direction === 'prev' ? 'left: 0.3125rem;' : 'right: 0.3125rem;')} /* 5px */
    width: 0;
    height: 0;
    border: 0.625rem solid transparent; /* 10px */
    ${(props) =>
      props.$direction === 'prev'
        ? 'border-right-color: #e2e2e2;'
        : 'border-left-color: #e2e2e2;'}
  }

  &:hover::before {
    ${(props) =>
      props.$direction === 'prev'
        ? 'border-right-color: #999;'
        : 'border-left-color: #999;'}
  }
`;
