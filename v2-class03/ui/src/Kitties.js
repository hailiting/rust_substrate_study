import React, { useEffect, useState } from "react";
import { Form, Grid, Card, Statistic } from "semantic-ui-react";
import { useSubstrate, useSubstrateState } from "./substrate-lib";
import { TxButton } from "./substrate-lib/components";
import KittyCards from "./KittyCards";
// - Pallet Kitties 的单元测试，尽量覆盖所有的方法和错误
// - [基于 Kitties-course 前端项目](https://github.com/SubstrateCourse/advance-5)
// - 能创建一个毛孩
// - 每一个毛孩展示成一张卡片，并显示是不是属于你的
// - 可以转让毛孩给另一位用户
// - [demo](https://www.awesomescreenshot.com/embed?id=2196893&shareKey=7749c0f9101a5791240bda8a391a1ce9)

export default function Main(props) {
  const { api } = useSubstrateState();
  const {
    state: { currentAccount: accountPair },
  } = useSubstrate();
  const [kittyCnt, setKittyCnt] = useState(0);
  const [, setStatus] = useState("");
  const [kittyOwnerList, setKittyOwnerList] = useState([]);
  const [kittyDNAList, setKittyDNAList] = useState([]);
  const [kittyOnSell, setKittyOnSell] = useState([]);
  const [kittyList, setKittyList] = useState([]);
  useEffect(() => {
    let unsubscribe;
    api.query.kittiesModule
      .nextKittyId((newValue) => {
        const len = newValue.toNumber();
        if (!newValue.isNone) {
          setKittyCnt(len);
        }
        if (len) {
          api.query.kittiesModule.kittyOwner.multi(
            [...Array(len).keys()],
            (data) => {
              const tempData = [];
              data.map((row) => {
                if (row.isNone) {
                  tempData.push(null);
                } else {
                  const _kittyOwner = row.value.toHuman();
                  tempData.push(_kittyOwner);
                }
                return null;
              });
              setKittyOwnerList(tempData);
            }
          );

          api.query.kittiesModule.kitties.multi(
            [...Array(len).keys()],
            (data) => {
              const tempData = [];
              data.map((row) => {
                if (row.isNone) {
                  tempData.push(null);
                } else {
                  const _kittyDNA = row.value.toU8a();
                  tempData.push(_kittyDNA);
                }
                return null;
              });
              setKittyDNAList(tempData);
            }
          );
          api.query.kittiesModule.onSale.multi(
            [...Array(len).keys()],
            (data) => {
              const tempData = [];
              data.map((row) => {
                // if(row.isNone)
                if (row.isNone) {
                  tempData.push(null);
                } else {
                  const _kittyPrice = row.value.toNumber();
                  tempData.push(_kittyPrice);
                }
                return null;
              });
              console.log({ tempData });
              setKittyOnSell(tempData);
            }
          );
        }
      })
      .then((unsub) => {
        unsubscribe = unsub;
      })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [api.query.kittiesModule]);
  useEffect(() => {
    const _kitties = [];
    if (kittyDNAList && kittyDNAList.length) {
      for (let i = 0; i < kittyDNAList.length; i++) {
        _kitties.push({
          id: i,
          dna: kittyDNAList[i],
          owner: kittyOwnerList[i],
          price: kittyOnSell[i],
        });
      }
      setKittyList(_kitties);
    }
  }, [kittyDNAList, kittyOwnerList, kittyOnSell]);
  return (
    <Grid.Column width={16}>
      <h1>小毛孩</h1>
      <Card centered>
        <Card.Content textAlign="center">
          <Statistic label="Next kitty id" value={kittyCnt} />
        </Card.Content>
      </Card>
      <Form style={{ margin: "1em 0" }}>
        <Form.Field style={{ textAlign: "center" }}>
          <TxButton
            label={"Create"}
            setStatus={setStatus}
            type="SIGNED-TX"
            attrs={{
              palletRpc: "kittiesModule",
              callable: "create",
              inputParams: [],
              paramFields: [],
            }}
          />
        </Form.Field>
      </Form>
      {/* <div style={{ overflowWrap: "break-word" }}>{status}</div> */}
      <KittyCards
        kitties={kittyList}
        accountPair={accountPair}
        setStatus={setStatus}
      />
    </Grid.Column>
  );
}
