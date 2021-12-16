use bitreader::BitReader;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<u8> {
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap()
}

#[derive(Debug)]
pub struct Packet {
    version: u8,
    data: PacketData,
}

#[derive(Debug)]
pub enum PacketData {
    Literal { value: u64 },
    Operator { type_id: OperatorType, length: OperatorLength, packets: Vec<Packet> },
}

#[derive(Debug)]
pub enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
pub enum OperatorLength {
    Bits(u16),
    PacketCount(u16),
}

fn read_number(reader: &mut BitReader) -> bitreader::Result<u64> {
    let mut result = 0u64;
    loop {
        let more = reader.read_bool()?;
        let bits = reader.read_u64(4)?;
        result = (result << 4) | bits;
        if !more { break; }
    }
    Ok(result)
}

impl Packet {
    pub fn parse(reader: &mut BitReader) -> bitreader::Result<Self> {
        let version = reader.read_u8(3)?;
        let type_id = reader.read_u8(3)?;
        let data = match type_id {
            4 => Self::parse_literal(reader)?,
            id => Self::parse_operator(reader, id)?
        };
        Ok(Self { version, data })
    }

    fn parse_literal(reader: &mut BitReader) -> bitreader::Result<PacketData> {
        let value = read_number(reader)?;
        Ok(PacketData::Literal { value })
    }

    fn parse_operator(reader: &mut BitReader, type_id: u8) -> bitreader::Result<PacketData> {
        let type_id = Self::parse_operator_type(type_id);
        let length_type_id = reader.read_bool()?;
        let length = match length_type_id {
            false => OperatorLength::Bits(reader.read_u16(15)?),
            true => OperatorLength::PacketCount(reader.read_u16(11)?),
        };
        let packets = match length {
            OperatorLength::Bits(bits) => Self::parse_sub_packets_by_bits(reader, bits)?,
            OperatorLength::PacketCount(count) => Self::parse_sub_packets_by_count(reader, count)?,
        };
        Ok(PacketData::Operator { type_id, length, packets })
    }

    fn parse_operator_type(type_id: u8) -> OperatorType {
        match type_id {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            // 4 => literal
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => panic!("unknown operator type: {}", type_id)
        }
    }

    fn parse_sub_packets_by_bits(reader: &mut BitReader, bits: u16) -> bitreader::Result<Vec<Packet>> {
        let mut packets = vec![];
        let start = reader.position();
        let bits = bits as u64;
        while reader.position() - start < bits {
            packets.push(Self::parse(reader)?);
        }
        assert_eq!(reader.position() - start, bits);
        Ok(packets)
    }

    fn parse_sub_packets_by_count(reader: &mut BitReader, count: u16) -> bitreader::Result<Vec<Packet>> {
        let packets = (0..count).map(|_| Self::parse(reader)).collect::<bitreader::Result<Vec<_>>>()?;
        Ok(packets)
    }

    pub fn sum_versions(&self) -> u64 {
        let mut sum = self.version as u64;
        sum += match &self.data {
            PacketData::Literal { .. } => 0,
            PacketData::Operator { packets, .. } => {
                packets.iter().map(|sub_packet| sub_packet.sum_versions()).sum()
            }
        };
        sum
    }

    pub fn evaluate(&self) -> u64 {
        match &self.data {
            PacketData::Literal { value } => { *value as u64 }
            PacketData::Operator { type_id, packets, .. } => {
                let mut values = packets.iter().map(|sub_packet| sub_packet.evaluate());
                match type_id {
                    OperatorType::Sum => values.sum(),
                    OperatorType::Product => values.product(),
                    OperatorType::Minimum => values.min().unwrap(),
                    OperatorType::Maximum => values.max().unwrap(),
                    OperatorType::GreaterThan => if values.next().unwrap() > values.next().unwrap() { 1 } else { 0 },
                    OperatorType::LessThan => if values.next().unwrap() < values.next().unwrap() { 1 } else { 0 },
                    OperatorType::EqualTo => if values.next().unwrap() == values.next().unwrap() { 1 } else { 0 },
                }
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &[u8]) -> u64 {
    let mut reader = BitReader::new(input);
    let packet = Packet::parse(&mut reader).unwrap();
    packet.sum_versions()
}

#[aoc(day16, part2)]
pub fn part2(input: &[u8]) -> u64 {
    let mut reader = BitReader::new(input);
    let packet = Packet::parse(&mut reader).unwrap();
    packet.evaluate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(&"8A004A801A8002F478")), 16);
        assert_eq!(part1(&input_generator(&"620080001611562C8802118E34")), 12);
        assert_eq!(part1(&input_generator(&"C0015000016115A2E0802F182340")), 23);
        assert_eq!(part1(&input_generator(&"A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(&"C200B40A82")), 3);
        assert_eq!(part2(&input_generator(&"04005AC33890")), 54);
        assert_eq!(part2(&input_generator(&"880086C3E88112")), 7);
        assert_eq!(part2(&input_generator(&"CE00C43D881120")), 9);
        assert_eq!(part2(&input_generator(&"D8005AC2A8F0")), 1);
        assert_eq!(part2(&input_generator(&"F600BC2D8F")), 0);
        assert_eq!(part2(&input_generator(&"9C005AC2F8F0")), 0);
        assert_eq!(part2(&input_generator(&"9C0141080250320F1802104A08")), 1);
    }
}
