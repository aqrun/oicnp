use crate::error::{CacheError, Result};
use crate::metadata::{CompressionAlgorithm, CompressionInfo};
use bytes::Bytes;

/// 压缩数据
/// 
/// 返回 `Bytes` 类型，便于零拷贝 Clone
pub fn compress(
    data: &[u8],
    algorithm: CompressionAlgorithm,
) -> Result<(Bytes, CompressionInfo)> {
    match algorithm {
        CompressionAlgorithm::Gzip => compress_gzip(data),
        CompressionAlgorithm::Zstd => compress_zstd(data),
        CompressionAlgorithm::Brotli => compress_brotli(data),
    }
}

/// 解压数据
/// 
/// 返回 `Bytes` 类型，便于零拷贝 Clone
pub fn decompress(
    compressed_data: &[u8],
    compression_info: &CompressionInfo,
) -> Result<Bytes> {
    match compression_info {
        CompressionInfo::None => Ok(Bytes::copy_from_slice(compressed_data)),
        CompressionInfo::Compressed { algorithm, .. } => {
            let decompressed = match algorithm {
                CompressionAlgorithm::Gzip => decompress_gzip(compressed_data)?,
                CompressionAlgorithm::Zstd => decompress_zstd(compressed_data)?,
                CompressionAlgorithm::Brotli => decompress_brotli(compressed_data)?,
            };
            Ok(Bytes::from(decompressed))
        }
    }
}

fn compress_gzip(data: &[u8]) -> Result<(Bytes, CompressionInfo)> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)
        .map_err(|e| CacheError::Compression(format!("Gzip compression failed: {}", e)))?;
    let compressed = encoder.finish()
        .map_err(|e| CacheError::Compression(format!("Gzip compression finish failed: {}", e)))?;
    
    let info = CompressionInfo::Compressed {
        original_size: data.len() as u64,
        compressed_size: compressed.len() as u64,
        algorithm: CompressionAlgorithm::Gzip,
    };
    
    Ok((Bytes::from(compressed), info))
}

fn decompress_gzip(compressed_data: &[u8]) -> Result<Vec<u8>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| CacheError::Compression(format!("Gzip decompression failed: {}", e)))?;
    
    Ok(decompressed)
}

fn compress_zstd(data: &[u8]) -> Result<(Bytes, CompressionInfo)> {
    let compressed = zstd::encode_all(data, 3)
        .map_err(|e| CacheError::Compression(format!("Zstd compression failed: {}", e)))?;
    
    let info = CompressionInfo::Compressed {
        original_size: data.len() as u64,
        compressed_size: compressed.len() as u64,
        algorithm: CompressionAlgorithm::Zstd,
    };
    
    Ok((Bytes::from(compressed), info))
}

fn decompress_zstd(compressed_data: &[u8]) -> Result<Vec<u8>> {
    let decompressed = zstd::decode_all(compressed_data)
        .map_err(|e| CacheError::Compression(format!("Zstd decompression failed: {}", e)))?;
    
    Ok(decompressed)
}

fn compress_brotli(data: &[u8]) -> Result<(Bytes, CompressionInfo)> {
    use std::io::Write;
    
    let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 6, 22);
    writer.write_all(data)
        .map_err(|e| CacheError::Compression(format!("Brotli compression failed: {}", e)))?;
    let compressed = writer.into_inner();
    
    let info = CompressionInfo::Compressed {
        original_size: data.len() as u64,
        compressed_size: compressed.len() as u64,
        algorithm: CompressionAlgorithm::Brotli,
    };
    
    Ok((Bytes::from(compressed), info))
}

fn decompress_brotli(compressed_data: &[u8]) -> Result<Vec<u8>> {
    use std::io::Read;
    
    let mut reader = brotli::Decompressor::new(compressed_data, 4096);
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed)
        .map_err(|e| CacheError::Compression(format!("Brotli decompression failed: {}", e)))?;
    
    Ok(decompressed)
}

