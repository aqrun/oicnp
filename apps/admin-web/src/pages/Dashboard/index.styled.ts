import styled from 'styled-components';

export const Container = styled.div`
  .overview {
    .ant-card-body {
      padding: 20px 24px 8px !important;
    }
    > * {
      box-sizing: border-box;
    }
    &-header {
      position: relative;
      width: 100%;
      overflow: hidden;
      &-meta {
        height: 22px;
        font-size: 14px;
        line-height: 22px;
      }
      &-count {
        height: 38px;
        margin-top: 4px;
        margin-bottom: 0;
        overflow: hidden;
        font-size: 30px;
        line-height: 38px;
        white-space: nowrap;
        text-overflow: ellipsis;
        word-break: break-all;
      }
      &-action {
        position: absolute;
        top: 4px;
        right: 0;
        line-height: 1;
        cursor: pointer;
      }
    }

    &-body {
      height: 46px;
      margin-bottom: 12px;
      position: relative;
    }

    &-footer {
      margin-top: 8px;
      padding-top: 9px;
      border-top: 1px solid #292a2d;
    }
  }

  .trend {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    &-item {
      display: inline-block;
      font-size: 14px;
      line-height: 22px;
      &:first-child {
        margin-right: 16px;
      }
      svg {
        vertical-align: middle;
      }
      > * {
        margin-right: 8px;
        &:nth-child(2) {
          color: rgba(0, 0, 0, 0.85);
        }
      }
    }
  }

  .field {
    font-size: 14px;
    line-height: 22px;
    // &-label {
    // }
    &-number {
      margin-left: 8px;
      color: rgba(0, 0, 0, 0.85);
    }
  }

  .salePercent {
    margin-top: 24px;

    .ant-list-item {
      padding: 8px 12px !important;
      > * {
        margin: 0 4px;
      }
    }
  }

  .customTooltip {
    transition: visibility 0.2s cubic-bezier(0.23, 1, 0.32, 1) 0s, left 0.4s cubic-bezier(0.23, 1, 0.32, 1) 0s,
      top 0.4s cubic-bezier(0.23, 1, 0.32, 1) 0s;
    background-color: rgba(256, 2560, 256, 0.9);
    box-shadow: rgb(174, 174, 174) 0px 0px 10px;
    border-radius: 3px;
    color: rgb(87, 87, 87);
    font-size: 12px;
    line-height: 20px;
    padding: 10px 10px 6px;
    &-titile {
      margin-bottom: 4px;
    }
    &-content {
      margin: 0px;
      list-style-type: none;
      padding: 0px;
      > li {
        margin-bottom: 4px;
      }
    }
  }

`;