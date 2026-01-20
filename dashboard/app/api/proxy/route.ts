import { NextRequest, NextResponse } from 'next/server'

const BOT_API_URL = process.env.BOT_API_URL || 'http://localhost:8080'

export async function GET(
  request: NextRequest,
  { params }: { params: { path: string[] } }
) {
  const path = request.nextUrl.searchParams.get('path') || 'status'
  
  try {
    const response = await fetch(`${BOT_API_URL}/api/${path}`, {
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      return NextResponse.json(
        { error: 'Failed to fetch from bot API' },
        { status: response.status }
      )
    }

    const data = await response.json()
    return NextResponse.json(data)
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to connect to bot API' },
      { status: 500 }
    )
  }
}
