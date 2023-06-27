"use client"

import { useState, useMemo } from 'react';
import { Container } from './index.styled';
import { Form, Input, Card, Row, Col, Alert } from 'antd';
import { FieldData } from 'rc-field-form/lib/interface';
import { MenuId } from '~/typings';
import {
  LayoutFooter,
  HtmlHead,
  SideBar,
  Header,
} from '~/components';

/**
 *
  高速过路费 34 
  河池寨 到 黑河森林公园 137公里
  平均油耗 7L/100km
  92油价7.37
  137公里 * 7 / 100 = 9.59L * 7.37 = 70.678 
  油费+过路费: 70.678+34 = 104.678

  油费过路总结:
  去 104.678 / 3人 = 34
  回 104.678 / 4人 = 26
*/
const Fares = () => {
  const [form] = Form.useForm();
  const [formValues, setFormValues] = useState<Record<string, string | number>>({});

  const fieldsChangeHandle = (changedFields: FieldData[], allFields: FieldData[]) => {
    const values = allFields?.reduce((obj: Record<string, string | number>, n: FieldData) => {
      const name = (n?.name as string[])?.[0];
      const newObj = obj;
      if (name) {
        newObj[name] = n?.value;
      }
      
      return newObj;
    }, {});
    setFormValues(values);
  }

  // 油费
  const oilCost = useMemo(() => {
    const miles = Number(formValues?.miles || 0);
    const fuelConsumptionPer100 = Number(formValues?.fuelConsumptionPer100 || 0);
    const oilPrice = Number(formValues?.oilPrice || 0);

    const label = `${miles || '[里程]'} * ${fuelConsumptionPer100 || '[百公里油耗]'} / 100 * ${oilPrice || '[当前油价]'}`;
    const value = miles * fuelConsumptionPer100 / 100 * oilPrice;
    return {
      label,
      value,
    };
  }, [formValues]);

  const costPerPassenger = useMemo(() => {
    const highWayPrice = Number(formValues?.highWayPrice || 0);
    const passengerNum = Number(formValues?.passengerNum || 0);
    let label = `(油费 + 高速过路费) / 人数`;
    let value = 0;

    if (!oilCost?.value || !passengerNum || !highWayPrice) return { label, value };

    label = `(${oilCost?.value || '[油费]'} + ${highWayPrice || '高速过路费'}) / ${passengerNum || '人数'}`;

    const res = (oilCost?.value + (highWayPrice || 0)) / (passengerNum || 0);
    value = Number(res.toFixed(3));
    return { label, value };
  }, [formValues, oilCost]);

  return (
    <Container className="oic-fare-container">
      <HtmlHead />
      {/*<Header menuId={MenuId.index} />*/}
      <main className="mx-[24px] my-12 md:mx-auto md:max-w-7xl">
        <Form
          form={form}
          onFieldsChange={fieldsChangeHandle}
        >
          <Row gutter={8}>
            <Col>
              <Form.Item
                label="起点"
                name="startLocation"
              >
                <Input placeholder="请输入起点" />
              </Form.Item>
            </Col>
            <Col>
              <Form.Item
                label="终点"
                name="endLocation"
              >
                <Input placeholder="请输入终点" />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={8}>
            <Col>
              <Form.Item
                label="里程(公里km)"
                name="miles"
              >
                <Input placeholder="请输入里程" />
              </Form.Item>
            </Col>
            <Col>
              <Form.Item
                label="高速过路费(元)"
                name="highWayPrice"
              >
                <Input placeholder="请输入高速过路费" />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={8}>
            <Col>
              <Form.Item
                label="百公里油耗(升/百公里 xL/100km)"
                name="fuelConsumptionPer100"
              >
                <Input placeholder="请输入百公里油耗" />
              </Form.Item>
            </Col>
            <Col>
              <Form.Item
                label="当前油价(元)"
                name="oilPrice"
              >
                <Input placeholder="请输入前油价" />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={8}>
            <Col>
              <Form.Item
                label="乘客数(人)"
                name="passengerNum"
              >
                <Input placeholder="请输入乘客数" />
              </Form.Item>
            </Col>
          </Row>
        </Form>

        <Alert
          type="success"
          className="mb-4"
          message={(
            <p>
              油费计算公式： 里程 * 百公里油耗 / 100 * 当前油价 <br/>
              平均到人计算公式：(油费 + 高速过路费) / 人数
            </p>
          )}
        />

        <Card
          title="油费过路费总结："
        >
          <div>
            从 {formValues?.startLocation || '[起点]'} 到 {formValues?.endLocation || '[终点]'}
          </div>
          <div>
            <span className="inline-block mr-2">油费:</span>
            <span>
              {oilCost?.label}
              <span className="inline-block mx-2">
                =
              </span>
              {oilCost?.value || '?'}
            </span>
          </div>
          <div>
            <span>每人平均费用:</span>
            <span>{costPerPassenger?.label}</span>
            <span className="inline-block mx-2">
              =
            </span>
            <span>{costPerPassenger?.value || '?'}</span>
          </div>
        </Card>
      </main>

      <LayoutFooter />
    </Container>
  );
}

export default Fares;
