import type { FC } from 'react';

import { Badge, Card } from 'antd';
import dayjs from 'dayjs';
import { Brush, CartesianGrid, Legend, Line, LineChart, ResponsiveContainer, Tooltip, XAxis, YAxis } from 'recharts';

const data = new Array(20).fill(null).map((_, index) => ({
  name: dayjs()
    .add(index * 30, 'minute')
    .format('HH:mm'),
  traffic: Math.floor(Math.random() * 120 + 1),
  payments: Math.floor(Math.random() * 120 + 1),
}));

interface CustomTooltipProps {
  active?: string;
  payload?: {
    value?: string;
    stroke?: string;
  }[];
  label?: string;
}

const CustomTooltip: FC<CustomTooltipProps> = ({ active, payload, label }) => {
  if (active) {
    const { value: value1, stroke: stroke1 } = payload?.[0] || {};
    const { value: value2, stroke: stroke2 } = payload?.[1] || {};

    return (
      <div className="customTooltip">
        <span className="customTooltip-title">{label}</span>
        <ul className="customTooltip-content">
          <li key="traffic">
            <Badge color={stroke1} />
            交通 {value1}
          </li>
          <li key="payments">
            <Badge color={stroke2} />
            支付 {value2}
          </li>
        </ul>
      </div>
    );
  }

  return null;
};

const TimeLine: FC<{ loading: boolean }> = ({ loading }) => {
  return (
    <Card
      loading={loading}
      style={{ marginTop: 24 }}
    >
      <ResponsiveContainer height={400}>
        <LineChart data={data} syncId="anyId">
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="name" />
          <YAxis />
          <Tooltip content={<CustomTooltip />} />
          <Line type="monotone" dataKey="traffic" stroke="#3F90F7" />
          <Line type="monotone" dataKey="payments" stroke="#61BE82" />
          <Brush dataKey="name" fill="#13c2c2" />
          <Legend
            verticalAlign="top"
            height={40}
            formatter={value => value}
          />
        </LineChart>
      </ResponsiveContainer>
    </Card>
  );
};

export default TimeLine;
