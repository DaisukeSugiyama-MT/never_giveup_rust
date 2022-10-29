import { useState } from "react";
import * as wasm from "../pkg";
import json from "../data/address.json";

type AddressList = {
  address_list: Address[];
};

type Address = {
  address1: String;
  address2: String;
  address3: String;
  address: String;
  kana1: String;
  kana2: String;
  kana3: String;
  n1: String;
  n2: String;
  n3: String;
  n4: String;
  n5: String;
  n6: String;
  pref: String;
  zip: String;
  zipcode: String;
};

const searchAddress = (value: string) => {
  const addressJson = json as AddressList;
  return addressJson.address_list.filter((address: Address) => {
    return address.zipcode.includes(value);
  });
};

const useHooks = () => {
  const [address, setAddress] = useState<Address[]>([]);
  const [text, setText] = useState<string>("");

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = e.currentTarget;
    setText(value);
    const startTime = Date.now();
    const res = wasm.search_address(value);
    //const res = searchAddress(value);
    const endTime = Date.now();
    console.log(`js search time: ${endTime - startTime}ms`);
    setAddress(res);
  };

  const sliced = address.length < 10 ? address : address.slice(0, 10);

  return { onChange, address: sliced, text };
};

export default useHooks;
