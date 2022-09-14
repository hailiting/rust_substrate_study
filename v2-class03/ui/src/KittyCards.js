import React from "react";
import {
  Button,
  Card,
  Grid,
  Message,
  Modal,
  Form,
  Label,
} from "semantic-ui-react";

import KittyAvatar from "./KittyAvatar";
import { TxButton } from "./substrate-lib/components";

// --- About Modal ---

const MyModal = (props) => {
  const {
    kitty,
    accountPair,
    setStatus,
    title,
    label,
    placeholder,
    callable,
    inputParams,
    paramFields,
    formChange,
    target,
  } = props;
  const [open, setOpen] = React.useState(false);

  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };
  return (
    <Modal
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
      open={open}
      trigger={
        <Button basic color="blue">
          {title}
        </Button>
      }
    >
      <Modal.Header>{title}</Modal.Header>
      <Modal.Content>
        <Form>
          <Form.Input fluid label="毛孩 ID" readOnly value={kitty.id} />
          {target ? (
            <Form.Input
              fluid
              label={label}
              placeholder={placeholder}
              onChange={formChange(target)}
            />
          ) : null}
        </Form>
      </Modal.Content>
      <Modal.Actions>
        <Button basic color="grey" onClick={() => setOpen(false)}>
          取消
        </Button>
        <TxButton
          accountPair={accountPair}
          label={`Comfirm ${title}`}
          type="SIGNED-TX"
          setStatus={setStatus}
          onClick={confirmAndClose}
          attrs={{
            palletRpc: "kittiesModule",
            callable,
            inputParams,
            paramFields,
          }}
        />
      </Modal.Actions>
    </Modal>
  );
};

// --- About Kitty Card ---

const KittyCard = (props) => {
  const [formValue, setFormValue] = React.useState({});
  const formChange = (key) => (ev, el) => {
    setFormValue({ ...formValue, [key]: el.value });
  };

  const { kitty, accountPair, setStatus } = props;
  const { id = null, dna = null, owner = null } = kitty;
  const displayDna = dna && dna.join(", ");
  const displayId = id === null ? "" : id < 10 ? `0${id}` : id.toString();
  const isSelf = accountPair.address === kitty.owner;

  return (
    <Card>
      {isSelf && (
        <Label as="a" floating color="teal">
          我的
        </Label>
      )}
      <KittyAvatar dna={dna} />
      <Card.Content>
        <Card.Header>ID 号: {displayId}</Card.Header>
        <Card.Meta style={{ overflowWrap: "break-word" }}>
          基因: <br />
          {displayDna}
        </Card.Meta>
        <Card.Description>
          <p style={{ overflowWrap: "break-word" }}>
            所有者:
            <br />
            {owner}
          </p>
        </Card.Description>
      </Card.Content>
      <Card.Content extra style={{ textAlign: "center" }}>
        {owner === accountPair.address ? (
          <>
            <MyModal
              title="Transfer"
              kitty={kitty}
              accountPair={accountPair}
              setStatus={setStatus}
              label="转让对象"
              placeholder="对方地址"
              callable="transfer"
              inputParams={[kitty.id, formValue.address]}
              paramFields={[true, true]}
              formChange={formChange}
              target="address"
            />

            <MyModal
              title={kitty.price ? "Change Price" : "Sell"}
              kitty={kitty}
              accountPair={accountPair}
              setStatus={setStatus}
              label="Sell price"
              placeholder="price"
              callable="sell"
              inputParams={[kitty.id, formValue.price]}
              paramFields={[true, true]}
              formChange={formChange}
              target="price"
            />
          </>
        ) : kitty.price ? (
          <MyModal
            title="Buy"
            kitty={kitty}
            accountPair={accountPair}
            setStatus={setStatus}
            callable="buy"
            inputParams={[kitty.id]}
            paramFields={[true]}
          />
        ) : null}
      </Card.Content>
    </Card>
  );
};

const KittyCards = (props) => {
  const { kitties, accountPair, setStatus } = props;

  if (kitties.length === 0) {
    return (
      <Message info>
        <Message.Header>
          现在连一只毛孩都木有，赶快创建一只&nbsp;
          <span role="img" aria-label="point-down">
            👇
          </span>
        </Message.Header>
      </Message>
    );
  }

  return (
    <Grid columns={3}>
      {kitties.map((kitty, i) => (
        <Grid.Column key={`kitty-${i}`}>
          <KittyCard
            kitty={kitty}
            accountPair={accountPair}
            setStatus={setStatus}
          />
        </Grid.Column>
      ))}
    </Grid>
  );
};

export default KittyCards;
