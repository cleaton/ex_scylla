defmodule ExScylla.CqlTypesUnitTest do
  use ExUnit.Case, async: true
  alias ExScylla.CqlTypes

  describe "primitives" do
    test ":int (32-bit signed integer)" do
      assert CqlTypes.decode_value(<<0::signed-32>>, :int) == 0
      assert CqlTypes.decode_value(<<42::signed-32>>, :int) == 42
      assert CqlTypes.decode_value(<<-100::signed-32>>, :int) == -100
      assert CqlTypes.decode_value(<<2_147_483_647::signed-32>>, :int) == 2_147_483_647
      assert CqlTypes.decode_value(<<-2_147_483_648::signed-32>>, :int) == -2_147_483_648
    end

    test ":big_int (64-bit signed integer)" do
      assert CqlTypes.decode_value(<<0::signed-64>>, :big_int) == 0

      assert CqlTypes.decode_value(<<123_456_789_012_345::signed-64>>, :big_int) ==
               123_456_789_012_345

      assert CqlTypes.decode_value(<<-9_876_543_210_987::signed-64>>, :big_int) ==
               -9_876_543_210_987

      assert CqlTypes.decode_value(<<9_223_372_036_854_775_807::signed-64>>, :big_int) ==
               9_223_372_036_854_775_807

      assert CqlTypes.decode_value(<<-9_223_372_036_854_775_808::signed-64>>, :big_int) ==
               -9_223_372_036_854_775_808
    end

    test ":small_int (16-bit signed integer)" do
      assert CqlTypes.decode_value(<<0::signed-16>>, :small_int) == 0
      assert CqlTypes.decode_value(<<32_767::signed-16>>, :small_int) == 32_767
      assert CqlTypes.decode_value(<<-32_768::signed-16>>, :small_int) == -32_768
      assert CqlTypes.decode_value(<<1234::signed-16>>, :small_int) == 1234
      assert CqlTypes.decode_value(<<-1234::signed-16>>, :small_int) == -1234
    end

    test ":tiny_int (8-bit signed integer)" do
      assert CqlTypes.decode_value(<<0::signed-8>>, :tiny_int) == 0
      assert CqlTypes.decode_value(<<127::signed-8>>, :tiny_int) == 127
      assert CqlTypes.decode_value(<<-128::signed-8>>, :tiny_int) == -128
      assert CqlTypes.decode_value(<<42::signed-8>>, :tiny_int) == 42
      assert CqlTypes.decode_value(<<-42::signed-8>>, :tiny_int) == -42
    end

    test ":boolean (0 and 1)" do
      assert CqlTypes.decode_value(<<0>>, :boolean) == false
      assert CqlTypes.decode_value(<<1>>, :boolean) == true
    end

    test ":float (32-bit float)" do
      assert CqlTypes.decode_value(<<0.0::float-32>>, :float) == 0.0
      assert CqlTypes.decode_value(<<1.5::float-32>>, :float) == 1.5
      assert CqlTypes.decode_value(<<-3.125::float-32>>, :float) == -3.125
    end

    test ":double (64-bit float)" do
      assert CqlTypes.decode_value(<<0.0::float-64>>, :double) == 0.0
      assert CqlTypes.decode_value(<<3.141592653589793::float-64>>, :double) == 3.141592653589793
      assert CqlTypes.decode_value(<<-12345.6789::float-64>>, :double) == -12345.6789
    end

    test ":text" do
      assert CqlTypes.decode_value("", :text) == ""
      assert CqlTypes.decode_value("hello world", :text) == "hello world"
      assert CqlTypes.decode_value("unicøde test 🚀", :text) == "unicøde test 🚀"
    end

    test ":ascii" do
      assert CqlTypes.decode_value("", :ascii) == ""
      assert CqlTypes.decode_value("ascii text 123", :ascii) == "ascii text 123"
    end

    test ":blob" do
      assert CqlTypes.decode_value(<<>>, :blob) == <<>>
      assert CqlTypes.decode_value(<<1, 2, 3, 4>>, :blob) == <<1, 2, 3, 4>>
      assert CqlTypes.decode_value(<<0, 255, 128, 64>>, :blob) == <<0, 255, 128, 64>>
    end
  end

  describe "uuid and timeuuid" do
    test ":uuid" do
      uuid_bytes =
        <<0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
          0x77, 0x88>>

      assert CqlTypes.decode_value(uuid_bytes, :uuid) == "12345678-9abc-def0-1122-334455667788"

      nil_uuid = <<0::128>>
      assert CqlTypes.decode_value(nil_uuid, :uuid) == "00000000-0000-0000-0000-000000000000"

      max_uuid = <<0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF::128>>
      assert CqlTypes.decode_value(max_uuid, :uuid) == "ffffffff-ffff-ffff-ffff-ffffffffffff"
    end

    test ":timeuuid" do
      timeuuid_bytes =
        <<0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
          0x66, 0x77>>

      assert CqlTypes.decode_value(timeuuid_bytes, :timeuuid) ==
               "fedcba98-7654-3210-0011-223344556677"
    end
  end

  describe "inet" do
    test "IPv4 4-byte" do
      assert CqlTypes.decode_value(<<127, 0, 0, 1>>, :inet) == {127, 0, 0, 1}
      assert CqlTypes.decode_value(<<192, 168, 1, 254>>, :inet) == {192, 168, 1, 254}
      assert CqlTypes.decode_value(<<0, 0, 0, 0>>, :inet) == {0, 0, 0, 0}
      assert CqlTypes.decode_value(<<255, 255, 255, 255>>, :inet) == {255, 255, 255, 255}
    end

    test "IPv6 16-byte" do
      assert CqlTypes.decode_value(<<0::128>>, :inet) == {0, 0, 0, 0, 0, 0, 0, 0}

      loopback = <<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1>>
      assert CqlTypes.decode_value(loopback, :inet) == {0, 0, 0, 0, 0, 0, 0, 1}

      ipv6_bytes =
        <<0x20, 0x01, 0x0D, 0xB8, 0x85, 0xA3, 0x00, 0x00, 0x00, 0x00, 0x8A, 0x2E, 0x03, 0x70,
          0x73, 0x34>>

      assert CqlTypes.decode_value(ipv6_bytes, :inet) ==
               {0x2001, 0x0DB8, 0x85A3, 0x0000, 0x0000, 0x8A2E, 0x0370, 0x7334}
    end
  end

  describe "date, time, timestamp, counter" do
    test ":date (32-bit unsigned integer)" do
      assert CqlTypes.decode_value(<<0::unsigned-32>>, :date) == 0
      assert CqlTypes.decode_value(<<2_147_483_648::unsigned-32>>, :date) == 2_147_483_648
      assert CqlTypes.decode_value(<<4_294_967_295::unsigned-32>>, :date) == 4_294_967_295
      assert CqlTypes.decode_value(<<18250::unsigned-32>>, :date) == 18250
    end

    test ":time (64-bit signed integer nanoseconds since midnight)" do
      assert CqlTypes.decode_value(<<0::signed-64>>, :time) == 0
      assert CqlTypes.decode_value(<<3_600_000_000_000::signed-64>>, :time) == 3_600_000_000_000
      assert CqlTypes.decode_value(<<86_399_999_999_999::signed-64>>, :time) == 86_399_999_999_999
    end

    test ":timestamp (64-bit signed integer milliseconds since epoch)" do
      assert CqlTypes.decode_value(<<0::signed-64>>, :timestamp) == 0

      assert CqlTypes.decode_value(<<1_693_000_000_000::signed-64>>, :timestamp) ==
               1_693_000_000_000

      assert CqlTypes.decode_value(<<-1_000_000::signed-64>>, :timestamp) == -1_000_000
    end

    test ":counter (64-bit signed integer)" do
      assert CqlTypes.decode_value(<<0::signed-64>>, :counter) == 0
      assert CqlTypes.decode_value(<<100::signed-64>>, :counter) == 100
      assert CqlTypes.decode_value(<<-50::signed-64>>, :counter) == -50
    end
  end

  describe "varint and decimal" do
    test ":varint positive, negative, and zero" do
      # zero
      assert CqlTypes.decode_value(<<0>>, :varint) == "0"
      assert CqlTypes.decode_value(<<0, 0>>, :varint) == "0"

      # positive
      assert CqlTypes.decode_value(<<1>>, :varint) == "1"
      assert CqlTypes.decode_value(<<127>>, :varint) == "127"
      assert CqlTypes.decode_value(<<0, 128>>, :varint) == "128"
      assert CqlTypes.decode_value(<<1, 0>>, :varint) == "256"
      assert CqlTypes.decode_value(<<1, 2, 3>>, :varint) == "66051"

      # negative
      assert CqlTypes.decode_value(<<255>>, :varint) == "-1"
      assert CqlTypes.decode_value(<<0x80, 0x00>>, :varint) == "-32768"
      assert CqlTypes.decode_value(<<0xFF, 0xFE>>, :varint) == "-2"
    end

    test ":decimal with positive, negative, zero unscaled, and scale formatting" do
      # scale = 0, positive
      bin_zero_scale = <<0::signed-32, 12345::signed-32>>
      assert CqlTypes.decode_value(bin_zero_scale, :decimal) == "12345"

      # scale <= 0 (negative scale)
      bin_neg_scale = <<-2::signed-32, 12345::signed-32>>
      assert CqlTypes.decode_value(bin_neg_scale, :decimal) == "12345"

      # scale > 0, length > scale
      bin_scaled = <<2::signed-32, 12345::signed-32>>
      assert CqlTypes.decode_value(bin_scaled, :decimal) == "123.45"

      # scale > 0, scale > length with zero padding
      # unscaled = 5 (length 1), scale = 3 -> "0.005"
      bin_pad = <<3::signed-32, 5::signed-8>>
      assert CqlTypes.decode_value(bin_pad, :decimal) == "0.005"

      # unscaled = 42 (length 2), scale = 4 -> "0.0042"
      bin_pad2 = <<4::signed-32, 42::signed-8>>
      assert CqlTypes.decode_value(bin_pad2, :decimal) == "0.0042"

      # zero unscaled, scale = 0 -> "0"
      bin_zero_unscaled = <<0::signed-32, 0::signed-8>>
      assert CqlTypes.decode_value(bin_zero_unscaled, :decimal) == "0"

      # zero unscaled, scale = 2 -> "0.00"
      bin_zero_scaled = <<2::signed-32, 0::signed-8>>
      assert CqlTypes.decode_value(bin_zero_scaled, :decimal) == "0.00"
    end
  end

  describe "collections" do
    test "{:list, :text} with elements, null elements, and empty list" do
      # ["foo", "bar"]
      bin = <<2::signed-32, 3::signed-32, "foo"::binary, 3::signed-32, "bar"::binary>>
      assert CqlTypes.decode_value(bin, {:list, :text}) == ["foo", "bar"]

      # ["foo", nil] (with -1 null element)
      bin_with_null = <<2::signed-32, 3::signed-32, "foo"::binary, -1::signed-32>>
      assert CqlTypes.decode_value(bin_with_null, {:list, :text}) == ["foo", nil]

      # [nil, nil]
      bin_all_null = <<2::signed-32, -1::signed-32, -1::signed-32>>
      assert CqlTypes.decode_value(bin_all_null, {:list, :text}) == [nil, nil]

      # Empty list
      bin_empty = <<0::signed-32>>
      assert CqlTypes.decode_value(bin_empty, {:list, :text}) == []
    end

    test "{:set, :int} with elements, null elements, and empty set" do
      # [10, 20]
      bin = <<2::signed-32, 4::signed-32, 10::signed-32, 4::signed-32, 20::signed-32>>
      assert CqlTypes.decode_value(bin, {:set, :int}) == [10, 20]

      # [nil, 30] (with -1 null element)
      bin_with_null = <<2::signed-32, -1::signed-32, 4::signed-32, 30::signed-32>>
      assert CqlTypes.decode_value(bin_with_null, {:set, :int}) == [nil, 30]

      # Empty set
      bin_empty = <<0::signed-32>>
      assert CqlTypes.decode_value(bin_empty, {:set, :int}) == []
    end
  end

  describe "maps" do
    test "{:map, {:text, :int}} with keys, values, nulls, and empty map" do
      # %{"a" => 100, "b" => 200}
      bin = <<
        2::signed-32,
        1::signed-32,
        "a"::binary,
        4::signed-32,
        100::signed-32,
        1::signed-32,
        "b"::binary,
        4::signed-32,
        200::signed-32
      >>

      assert CqlTypes.decode_value(bin, {:map, {:text, :int}}) == %{"a" => 100, "b" => 200}

      # with null key and null value (-1 length)
      bin_with_nulls = <<
        2::signed-32,
        -1::signed-32,
        4::signed-32,
        100::signed-32,
        3::signed-32,
        "key"::binary,
        -1::signed-32
      >>

      assert CqlTypes.decode_value(bin_with_nulls, {:map, {:text, :int}}) == %{
               nil => 100,
               "key" => nil
             }

      # Empty map
      bin_empty = <<0::signed-32>>
      assert CqlTypes.decode_value(bin_empty, {:map, {:text, :int}}) == %{}
    end
  end

  describe "tuples" do
    test "{:tuple, [:int, :text]} with elements, nulls, and empty" do
      # {42, "hello"}
      bin = <<4::signed-32, 42::signed-32, 5::signed-32, "hello"::binary>>
      assert CqlTypes.decode_value(bin, {:tuple, [:int, :text]}) == {42, "hello"}

      # {nil, "world"}
      bin_with_null = <<-1::signed-32, 5::signed-32, "world"::binary>>
      assert CqlTypes.decode_value(bin_with_null, {:tuple, [:int, :text]}) == {nil, "world"}

      # {nil, nil}
      bin_all_null = <<-1::signed-32, -1::signed-32>>
      assert CqlTypes.decode_value(bin_all_null, {:tuple, [:int, :text]}) == {nil, nil}

      # Empty tuple
      assert CqlTypes.decode_value(<<>>, {:tuple, []}) == {}
      assert CqlTypes.decode_value(<<>>, {:tuple, [:int, :text]}) == {}
    end
  end

  describe "user defined types (UDTs)" do
    test "{:user_defined_type, fields} with values, nulls, and empty remainder" do
      fields = [{"f1", :int}, {"f2", :text}]

      # %{"f1" => 123, "f2" => "test"}
      bin = <<4::signed-32, 123::signed-32, 4::signed-32, "test"::binary>>

      assert CqlTypes.decode_value(bin, {:user_defined_type, fields}) == %{
               "f1" => 123,
               "f2" => "test"
             }

      # with null values (-1 length)
      bin_null = <<-1::signed-32, -1::signed-32>>

      assert CqlTypes.decode_value(bin_null, {:user_defined_type, fields}) == %{
               "f1" => nil,
               "f2" => nil
             }

      # partial fields / empty remaining binary
      bin_partial = <<4::signed-32, 999::signed-32>>
      assert CqlTypes.decode_value(bin_partial, {:user_defined_type, fields}) == %{"f1" => 999}

      # empty binary
      assert CqlTypes.decode_value(<<>>, {:user_defined_type, fields}) == %{}
      assert CqlTypes.decode_value(<<>>, {:user_defined_type, []}) == %{}
    end
  end

  describe "fallback unknown type" do
    test "returns raw binary unchanged for unrecognized types" do
      assert CqlTypes.decode_value(<<1, 2, 3, 4>>, :unknown_type) == <<1, 2, 3, 4>>
      assert CqlTypes.decode_value("custom_payload", {:custom, 42}) == "custom_payload"
    end
  end
end
