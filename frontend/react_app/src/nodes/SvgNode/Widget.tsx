import { SvgNodeModel } from "./Model";
import { DiagramEngine, PortModelAlignment, PortWidget } from "@projectstorm/react-diagrams";
import styled from '@emotion/styled';

export interface SvgWidgetProps {
    node: SvgNodeModel;
    engine: DiagramEngine;
    size: number;
}

namespace S {
    export const Port = styled.div`
        width: 16px;
        height: 16px;
        z-index: 10;
        background: rgba(0, 0, 0, 0.5);
        border-radius: 8px;
        cursor: pointer;
        &:hover {
            background: rgba(0, 0, 0, 1);
        }
    `;
}

export function SvgNodeWidget(props: SvgWidgetProps) {
    return (
        <div
            className={'diamond-node'}
            style={{
                position: 'relative',
                width: props.size,
                height: props.size
            }}>
            <div
                style={{
                    position: 'absolute',
                    top: props.size * 3 / 4,
                    left: 0,
                    width: props.size,
                    textAlign: 'center',
                    fontSize: 20,
                    fontWeight: 'bold'
                }}>
                {props.node.item.name}
            </div>
            <svg
                width={props.size}
                height={props.size}
                dangerouslySetInnerHTML={{
                    __html: props.node.item.svg
                }} />
            <PortWidget
                style={{
                    top: props.size / 2 - 7,
                    left: -7,
                    position: 'absolute'
                }}
                port={props.node.getPort(PortModelAlignment.LEFT)!}
                engine={props.engine}>
                <S.Port />
            </PortWidget>
            <PortWidget
                style={{
                    top: props.size / 2 - 7,
                    left: props.size - 7,
                    position: 'absolute'
                }}
                port={props.node.getPort(PortModelAlignment.RIGHT)!}
                engine={props.engine}>
                <S.Port />
            </PortWidget>
        </div>

    );
};